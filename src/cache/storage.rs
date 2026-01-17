use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::embedder::{cosine_similarity, Embedder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedQuery {
    pub id: i64,
    pub query_original: String,
    pub query_normalized: String,
    pub query_hash: String,
    pub response: String,
    pub provider: String,
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: i64,
    pub total_size_bytes: i64,
    pub hit_count: i64,
    pub miss_count: i64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}

pub struct CacheStorage {
    conn: Connection,
    cache_dir: PathBuf,
    embedder: Option<Embedder>,
}

impl CacheStorage {
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();

        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
        }

        let db_path = cache_dir.join("queries.db");
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open cache database: {}", db_path.display()))?;

        let embedder = Some(Embedder::new(Embedder::get_default_dimensions()));

        let storage = Self {
            conn,
            cache_dir,
            embedder,
        };
        storage.initialize_schema()?;

        Ok(storage)
    }

    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS queries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query_original TEXT NOT NULL,
                query_normalized TEXT NOT NULL,
                query_hash TEXT NOT NULL UNIQUE,
                embedding BLOB,
                response TEXT NOT NULL,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                last_accessed INTEGER NOT NULL,
                access_count INTEGER DEFAULT 1
            )",
            [],
        )?;

        let _ = self
            .conn
            .execute("ALTER TABLE queries ADD COLUMN embedding BLOB", []);

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_query_hash ON queries(query_hash)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON queries(created_at)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_access_count ON queries(access_count)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS cache_stats (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                hit_count INTEGER DEFAULT 0,
                miss_count INTEGER DEFAULT 0
            )",
            [],
        )?;

        self.conn.execute(
            "INSERT OR IGNORE INTO cache_stats (id, hit_count, miss_count) VALUES (1, 0, 0)",
            [],
        )?;

        Ok(())
    }

    pub fn store(
        &self,
        query_original: &str,
        query_normalized: &str,
        query_hash: &str,
        response: &str,
        provider: &str,
        model: &str,
    ) -> Result<i64> {
        let now = Utc::now().timestamp();

        let embedding_blob = if let Some(ref embedder) = self.embedder {
            let embedding = embedder.embed(query_normalized);
            Some(bincode::serialize(&embedding)?)
        } else {
            None
        };

        let _id = self.conn.execute(
            "INSERT INTO queries (
                query_original, query_normalized, query_hash, embedding, response,
                provider, model, created_at, last_accessed, access_count
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(query_hash) DO UPDATE SET
                embedding = excluded.embedding,
                response = excluded.response,
                provider = excluded.provider,
                model = excluded.model,
                last_accessed = excluded.last_accessed,
                access_count = access_count + 1",
            params![
                query_original,
                query_normalized,
                query_hash,
                embedding_blob,
                response,
                provider,
                model,
                now,
                now
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_by_hash(&self, query_hash: &str) -> Result<Option<CachedQuery>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, query_original, query_normalized, query_hash, response, embedding,
                    provider, model, created_at, last_accessed, access_count
             FROM queries WHERE query_hash = ?1",
        )?;

        let result = stmt.query_row(params![query_hash], |row| {
            Ok(CachedQuery {
                id: row.get(0)?,
                query_original: row.get(1)?,
                query_normalized: row.get(2)?,
                query_hash: row.get(3)?,
                response: row.get(4)?,
                provider: row.get(5)?,
                model: row.get(6)?,
                created_at: DateTime::from_timestamp(row.get(7)?, 0).unwrap_or_else(Utc::now),
                last_accessed: DateTime::from_timestamp(row.get(8)?, 0).unwrap_or_else(Utc::now),
                access_count: row.get(9)?,
            })
        });

        match result {
            Ok(cached) => {
                self.update_access(&cached.query_hash)?;
                self.increment_hit_count()?;
                Ok(Some(cached))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                self.increment_miss_count()?;
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn search_similar(
        &self,
        query_normalized: &str,
        threshold: f32,
        limit: usize,
    ) -> Result<Vec<(CachedQuery, f32)>> {
        let query_embedding = if let Some(ref embedder) = self.embedder {
            embedder.embed(query_normalized)
        } else {
            return Ok(Vec::new());
        };

        let mut stmt = self.conn.prepare(
            "SELECT id, query_original, query_normalized, query_hash, embedding, response,
                    provider, model, created_at, last_accessed, access_count
             FROM queries WHERE embedding IS NOT NULL",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                CachedQuery {
                    id: row.get(0)?,
                    query_original: row.get(1)?,
                    query_normalized: row.get(2)?,
                    query_hash: row.get(3)?,
                    response: row.get(5)?,
                    provider: row.get(6)?,
                    model: row.get(7)?,
                    created_at: DateTime::from_timestamp(row.get(8)?, 0).unwrap_or_else(Utc::now),
                    last_accessed: DateTime::from_timestamp(row.get(9)?, 0)
                        .unwrap_or_else(Utc::now),
                    access_count: row.get(10)?,
                },
                row.get::<_, Vec<u8>>(4)?,
            ))
        })?;

        let mut results: Vec<(CachedQuery, f32)> = Vec::new();
        for row_result in rows {
            let (cached_query, embedding_blob) = row_result?;

            if let Ok(cached_embedding) = bincode::deserialize::<Vec<f32>>(&embedding_blob) {
                let similarity = cosine_similarity(&query_embedding, &cached_embedding);

                if similarity >= threshold {
                    results.push((cached_query, similarity));
                }
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }

    fn update_access(&self, query_hash: &str) -> Result<()> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE queries SET last_accessed = ?1, access_count = access_count + 1
             WHERE query_hash = ?2",
            params![now, query_hash],
        )?;
        Ok(())
    }

    pub fn list_all(&self, limit: Option<usize>) -> Result<Vec<CachedQuery>> {
        let limit_clause = limit.map(|l| format!("LIMIT {}", l)).unwrap_or_default();

        let query = format!(
            "SELECT id, query_original, query_normalized, query_hash, response,
                    provider, model, created_at, last_accessed, access_count
             FROM queries ORDER BY last_accessed DESC {}",
            limit_clause
        );

        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            Ok(CachedQuery {
                id: row.get(0)?,
                query_original: row.get(1)?,
                query_normalized: row.get(2)?,
                query_hash: row.get(3)?,
                response: row.get(4)?,
                provider: row.get(5)?,
                model: row.get(6)?,
                created_at: DateTime::from_timestamp(row.get(7)?, 0).unwrap_or_else(Utc::now),
                last_accessed: DateTime::from_timestamp(row.get(8)?, 0).unwrap_or_else(Utc::now),
                access_count: row.get(9)?,
            })
        })?;

        let mut queries = Vec::new();
        for row in rows {
            queries.push(row?);
        }

        Ok(queries)
    }

    pub fn stats(&self) -> Result<CacheStats> {
        let total_entries: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM queries", [], |row| row.get(0))?;

        let total_size_bytes: i64 = self.conn.query_row(
            "SELECT COALESCE(SUM(LENGTH(response) + LENGTH(query_original)), 0) FROM queries",
            [],
            |row| row.get(0),
        )?;

        let (hit_count, miss_count): (i64, i64) = self.conn.query_row(
            "SELECT hit_count, miss_count FROM cache_stats WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        let oldest_entry: Option<i64> = self
            .conn
            .query_row("SELECT MIN(created_at) FROM queries", [], |row| row.get(0))
            .ok();

        let newest_entry: Option<i64> = self
            .conn
            .query_row("SELECT MAX(created_at) FROM queries", [], |row| row.get(0))
            .ok();

        Ok(CacheStats {
            total_entries,
            total_size_bytes,
            hit_count,
            miss_count,
            oldest_entry: oldest_entry.and_then(|ts| DateTime::from_timestamp(ts, 0)),
            newest_entry: newest_entry.and_then(|ts| DateTime::from_timestamp(ts, 0)),
        })
    }

    fn increment_hit_count(&self) -> Result<()> {
        self.conn.execute(
            "UPDATE cache_stats SET hit_count = hit_count + 1 WHERE id = 1",
            [],
        )?;
        Ok(())
    }

    fn increment_miss_count(&self) -> Result<()> {
        self.conn.execute(
            "UPDATE cache_stats SET miss_count = miss_count + 1 WHERE id = 1",
            [],
        )?;
        Ok(())
    }

    pub fn clear(&self) -> Result<usize> {
        let count = self.conn.execute("DELETE FROM queries", [])?;

        self.conn.execute(
            "UPDATE cache_stats SET hit_count = 0, miss_count = 0 WHERE id = 1",
            [],
        )?;

        Ok(count)
    }

    pub fn remove_by_hash(&self, query_hash: &str) -> Result<bool> {
        let count = self.conn.execute(
            "DELETE FROM queries WHERE query_hash = ?1",
            params![query_hash],
        )?;
        Ok(count > 0)
    }

    pub fn get_cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn cleanup_old_entries(&self, days: u32) -> Result<usize> {
        let cutoff = Utc::now().timestamp() - (days as i64 * 86400);

        let count = self
            .conn
            .execute("DELETE FROM queries WHERE created_at < ?1", params![cutoff])?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_storage() -> (CacheStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();
        (storage, temp_dir)
    }

    fn test_store_and_get() {
        let (storage, _temp) = create_test_storage();

        let hash = "testhash123";
        storage
            .store(
                "original query",
                "normalized query",
                hash,
                "test response",
                "TestProvider",
                "test-model",
            )
            .unwrap();

        let cached = storage.get_by_hash(hash).unwrap();
        assert!(cached.is_some());

        let cached = cached.unwrap();
        assert_eq!(cached.query_original, "original query");
        assert_eq!(cached.query_normalized, "normalized query");
        assert_eq!(cached.response, "test response");
        assert_eq!(cached.provider, "TestProvider");
        assert!(cached.access_count >= 1);
    }

    #[test]
    fn test_duplicate_hash_updates() {
        let (storage, _temp) = create_test_storage();

        let hash = "samehash";
        storage
            .store("query1", "norm1", hash, "response1", "P1", "m1")
            .unwrap();
        storage
            .store("query2", "norm2", hash, "response2", "P2", "m2")
            .unwrap();

        let cached = storage.get_by_hash(hash).unwrap().unwrap();
        assert_eq!(cached.response, "response2");
        assert_eq!(cached.provider, "P2");
        assert!(cached.access_count >= 2);
    }

    #[test]
    fn test_list_all() {
        let (storage, _temp) = create_test_storage();

        storage.store("q1", "n1", "h1", "r1", "p", "m").unwrap();
        storage.store("q2", "n2", "h2", "r2", "p", "m").unwrap();
        storage.store("q3", "n3", "h3", "r3", "p", "m").unwrap();

        let all = storage.list_all(None).unwrap();
        assert_eq!(all.len(), 3);

        let limited = storage.list_all(Some(2)).unwrap();
        assert_eq!(limited.len(), 2);
    }

    #[test]
    fn test_stats() {
        let (storage, _temp) = create_test_storage();

        storage
            .store("q1", "n1", "h1", "response1", "p", "m")
            .unwrap();
        storage
            .store("q2", "n2", "h2", "response2", "p", "m")
            .unwrap();

        let stats = storage.stats().unwrap();
        assert_eq!(stats.total_entries, 2);
        assert!(stats.total_size_bytes > 0);
        assert!(stats.newest_entry.is_some());
    }

    #[test]
    fn test_hit_miss_tracking() {
        let (storage, _temp) = create_test_storage();

        storage.store("q1", "n1", "h1", "r1", "p", "m").unwrap();

        storage.get_by_hash("h1").unwrap();
        storage.get_by_hash("nonexistent").unwrap();

        let stats = storage.stats().unwrap();
        assert_eq!(stats.hit_count, 1);
        assert_eq!(stats.miss_count, 1);
    }

    #[test]
    fn test_clear() {
        let (storage, _temp) = create_test_storage();

        storage.store("q1", "n1", "h1", "r1", "p", "m").unwrap();
        storage.store("q2", "n2", "h2", "r2", "p", "m").unwrap();

        let count = storage.clear().unwrap();
        assert_eq!(count, 2);

        let stats = storage.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.hit_count, 0);
        assert_eq!(stats.miss_count, 0);
    }

    #[test]
    fn test_remove_by_hash() {
        let (storage, _temp) = create_test_storage();

        storage.store("q1", "n1", "h1", "r1", "p", "m").unwrap();

        let removed = storage.remove_by_hash("h1").unwrap();
        assert!(removed);

        let cached = storage.get_by_hash("h1").unwrap();
        assert!(cached.is_none());

        let removed_again = storage.remove_by_hash("h1").unwrap();
        assert!(!removed_again);
    }

    #[test]
    fn test_access_count_increments() {
        let (storage, _temp) = create_test_storage();

        storage.store("q", "n", "h", "r", "p", "m").unwrap();

        for _ in 0..5 {
            storage.get_by_hash("h").unwrap();
        }

        let cached = storage.get_by_hash("h").unwrap().unwrap();
        assert!(cached.access_count >= 6);
    }
}
