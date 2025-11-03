pub mod embedder;
pub mod embedder_onnx;
pub mod normalizer;
pub mod storage;

pub use embedder::{cosine_similarity, Embedder, ModelInfo};
pub use embedder_onnx::ONNXEmbedder;
pub use normalizer::{NormalizationConfig, QueryNormalizer};
pub use storage::{CacheStats, CacheStorage, CachedQuery};
