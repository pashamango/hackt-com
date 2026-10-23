use crate::generator::GenerationResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CachedArtifact {
    pub result: GenerationResult,
    pub hits: u64,
    pub cached_at: String,
}

/// Content-addressed deduplication cache (SIMSIM / INFS compliant).
/// Prevents loop generation by indexing artifacts by their canonical envelope hash.
#[derive(Debug, Default)]
pub struct SimSimCache {
    entries: HashMap<String, CachedArtifact>,
}

impl SimSimCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Look up artifact by envelope hash. If found, returns a clone with `from_cache: true`.
    pub fn get(&mut self, envelope_hash: &str) -> Option<GenerationResult> {
        if let Some(cached) = self.entries.get_mut(envelope_hash) {
            cached.hits += 1;
            let mut out = cached.result.clone();
            out.from_cache = true;
            Some(out)
        } else {
            None
        }
    }

    /// Insert newly generated artifact.
    pub fn put(&mut self, envelope_hash: String, result: GenerationResult) {
        let entry = CachedArtifact {
            result,
            hits: 0,
            cached_at: "2026-10-23T00:00:00Z".to_string(),
        };
        self.entries.insert(envelope_hash, entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
