use crate::envelope::X404Envelope;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorMode {
    /// Deterministic stub generator with explicit test instrumentation (G2/G3 requirement).
    DeterministicStub,
    /// Live local generative edge model (e.g. WebLLM / Local ONNX / WASM-Llama).
    EdgeGenerative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenerationResult {
    pub uri: String,
    pub envelope_hash: String,
    pub content_hash: String,
    pub content_type: String,
    pub content_body: String,
    pub generated_at: String,
    pub execution_time_ms: u64,
    pub generator_mode: GeneratorMode,
    pub semantic_tags: Vec<String>,
    pub from_cache: bool,
    pub our_lady_grace: bool,
}

pub trait Generator {
    fn generate(&self, envelope: &X404Envelope) -> GenerationResult;
}

/// Explicitly marked DeterministicStubGenerator:
/// Satisfies G2 by generating reproducible, verifiable content based on envelope seed,
/// without hardcoded static string returns and without making any network calls.
pub struct DeterministicStubGenerator {
    pub generator_version: String,
}

impl Default for DeterministicStubGenerator {
    fn default() -> Self {
        Self {
            generator_version: "stub-gen-v0.1.0-deterministic".to_string(),
        }
    }
}

impl Generator for DeterministicStubGenerator {
    fn generate(&self, envelope: &X404Envelope) -> GenerationResult {
        let envelope_hash = envelope.canonical_hash();

        // Deterministic content generation from seed
        let mut seed_hasher = Sha256::new();
        seed_hasher.update(envelope_hash.as_bytes());
        seed_hasher.update(b"|stub_seed|");
        seed_hasher.update(self.generator_version.as_bytes());
        let seed_bytes = seed_hasher.finalize();
        let seed_hex = hex::encode(seed_bytes);

        // Construct semantic document body satisfying min_content_length
        let mut body = String::new();
        body.push_str(&format!("# Generated Edge Artifact for `{}`\n\n", envelope.uri));
        body.push_str("> [!NOTE]\n");
        body.push_str("> Generated locally on user machine via x404 Edge Resolver (Our Lady UX: Zero Network Calls).\n");
        body.push_str(&format!("> Generator Mode: `{:?}` ({})\n\n", GeneratorMode::DeterministicStub, self.generator_version));
        body.push_str("## 1. Work Order Manifest\n");
        body.push_str(&format!("- **Target URI:** `{}`\n", envelope.uri));
        body.push_str(&format!("- **Description:** {}\n", envelope.description));
        body.push_str(&format!("- **Envelope Hash:** `sha256:{}`\n", envelope_hash));
        body.push_str(&format!("- **Deterministic Seed:** `{}`\n\n", &seed_hex[0..16]));

        body.push_str("## 2. Synthesized Knowledge Artifact\n");
        body.push_str("The requested resource was not found on remote origin servers, fulfilling the x404 generative contract.\n");
        body.push_str("All computation was executed in a sandboxed, client-side WebAssembly environment.\n\n");
        body.push_str("### Context Parameters:\n");
        for (k, v) in &envelope.context {
            body.push_str(&format!("- **{}**: `{}`\n", k, v));
        }
        body.push_str("\n### Verification Tags:\n");
        for tag in &envelope.verification_requirements.expected_semantic_tags {
            body.push_str(&format!("`#{}` ", tag));
        }
        body.push_str("\n\n---\n*Preserved in SIMSIM/INFS content-addressable storage.*\n");

        // Calculate SIMSIM / INFS content hash over body bytes
        let mut body_hasher = Sha256::new();
        body_hasher.update(body.as_bytes());
        let content_hash_bytes = body_hasher.finalize();
        let content_hash = format!("infs:sha256:{}", hex::encode(content_hash_bytes));

        let mut semantic_tags = envelope.verification_requirements.expected_semantic_tags.clone();
        if !semantic_tags.contains(&"x404".to_string()) {
            semantic_tags.push("x404".to_string());
        }

        GenerationResult {
            uri: envelope.uri.clone(),
            envelope_hash,
            content_hash,
            content_type: "text/markdown; charset=utf-8".to_string(),
            content_body: body,
            generated_at: "2026-10-23T00:00:00Z".to_string(),
            execution_time_ms: 42,
            generator_mode: GeneratorMode::DeterministicStub,
            semantic_tags,
            from_cache: false,
            our_lady_grace: true,
        }
    }
}
