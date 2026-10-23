use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Policy {
    pub allowed_modalities: Vec<String>,
    pub max_compute_ms: u64,
    pub local_only: bool,
    pub grace_attestation: bool,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            allowed_modalities: vec![
                "text/markdown".to_string(),
                "application/json".to_string(),
                "text/html".to_string(),
            ],
            max_compute_ms: 15_000,
            local_only: true,
            grace_attestation: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationRequirements {
    pub min_content_length: usize,
    pub schema_url: Option<String>,
    pub expected_semantic_tags: Vec<String>,
    pub integrity_hash_algo: String,
}

impl Default for VerificationRequirements {
    fn default() -> Self {
        Self {
            min_content_length: 64,
            schema_url: None,
            expected_semantic_tags: vec!["x404".to_string(), "edge-generative".to_string()],
            integrity_hash_algo: "sha256".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct X404Envelope {
    pub envelope_version: String,
    pub uri: String,
    pub description: String,
    pub context: HashMap<String, String>,
    pub policy: Policy,
    pub verification_requirements: VerificationRequirements,
    #[serde(default = "default_created_at")]
    pub created_at: String,
    pub client_identity: Option<String>,
}

fn default_created_at() -> String {
    "2026-10-23T00:00:00Z".to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum X404Error {
    JsonParseError(String),
    ValidationError(String),
    InvalidPolicy(String),
}

impl std::fmt::Display for X404Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X404Error::JsonParseError(e) => write!(f, "x404 JSON parse error: {}", e),
            X404Error::ValidationError(e) => write!(f, "x404 validation error: {}", e),
            X404Error::InvalidPolicy(e) => write!(f, "x404 invalid policy: {}", e),
        }
    }
}

impl std::error::Error for X404Error {}

impl X404Envelope {
    pub fn parse(json_str: &str) -> Result<Self, X404Error> {
        let envelope: Self = serde_json::from_str(json_str)
            .map_err(|e| X404Error::JsonParseError(e.to_string()))?;
        envelope.validate()?;
        Ok(envelope)
    }

    pub fn validate(&self) -> Result<(), X404Error> {
        if self.uri.trim().is_empty() {
            return Err(X404Error::ValidationError("URI must not be empty".to_string()));
        }
        if self.description.trim().is_empty() {
            return Err(X404Error::ValidationError("Description must not be empty".to_string()));
        }
        if !self.policy.local_only {
            return Err(X404Error::InvalidPolicy("Edge-first invariant violation: policy.local_only must be true".to_string()));
        }
        if self.policy.allowed_modalities.is_empty() {
            return Err(X404Error::InvalidPolicy("Allowed modalities cannot be empty".to_string()));
        }
        Ok(())
    }

    /// Computes canonical semantic hash over the normalized request envelope fields
    /// for content-addressed SIMSIM deduplication.
    pub fn canonical_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.envelope_version.as_bytes());
        hasher.update(b"|");
        hasher.update(self.uri.trim().as_bytes());
        hasher.update(b"|");
        hasher.update(self.description.trim().as_bytes());
        hasher.update(b"|");

        // Deterministic context sorting
        let mut ctx_keys: Vec<_> = self.context.keys().collect();
        ctx_keys.sort();
        for k in ctx_keys {
            hasher.update(k.as_bytes());
            hasher.update(b"=");
            hasher.update(self.context[k].as_bytes());
            hasher.update(b";");
        }

        hasher.update(b"|");
        for m in &self.policy.allowed_modalities {
            hasher.update(m.as_bytes());
            hasher.update(b",");
        }

        let hash_bytes = hasher.finalize();
        hex::encode(hash_bytes)
    }
}
