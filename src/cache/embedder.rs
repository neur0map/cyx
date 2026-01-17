use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Embedder {
    dimensions: usize,
}

impl Embedder {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }

    pub fn embed(&self, text: &str) -> Vec<f32> {
        let normalized_text = text.to_lowercase();
        let words: Vec<&str> = normalized_text.split_whitespace().collect();

        let mut embedding = vec![0.0; self.dimensions];

        for (i, word) in words.iter().enumerate() {
            let word_hash = Self::hash_string(word) % self.dimensions;
            embedding[word_hash] += 1.0 / (i + 1) as f32;
        }

        for word in &words {
            if word.len() >= 3 {
                for i in 0..word.len() - 2 {
                    let trigram = &word[i..i + 3];
                    let trigram_hash = Self::hash_string(trigram) % self.dimensions;
                    embedding[trigram_hash] += 0.5;
                }
            }
        }

        let avg_word_len = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f32 / words.len() as f32
        } else {
            0.0
        };

        if self.dimensions > 10 {
            embedding[self.dimensions - 1] = avg_word_len / 10.0;
            embedding[self.dimensions - 2] = words.len() as f32 / 20.0;
        }

        Self::normalize_vector(&mut embedding);

        embedding
    }

    fn hash_string(s: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn normalize_vector(vec: &mut [f32]) {
        let norm: f32 = vec.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in vec.iter_mut() {
                *x /= norm;
            }
        }
    }

    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    pub const fn get_default_dimensions() -> usize {
        256
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![-1.0, -2.0, -3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - (-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_different_lengths() {
        let vec1 = vec![1.0, 2.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert_eq!(sim, 0.0);
    }
}
