pub mod cache;
pub mod envelope;
pub mod generator;
pub mod patience;

use cache::SimSimCache;
use envelope::X404Envelope;
use generator::{DeterministicStubGenerator, GenerationResult, Generator};
use patience::{is_fsck_off_command, FsckOffReceipt, X420Envelope};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "response_type", content = "payload")]
pub enum ResolverResponse {
    #[serde(rename = "generation_success")]
    Success(GenerationResult),
    #[serde(rename = "patience_backpressure")]
    Patience(X420Envelope),
    #[serde(rename = "fsck_off_receipt")]
    FsckOff(FsckOffReceipt),
    #[serde(rename = "error")]
    Error(String),
}

/// Core Edge-First x404 Resolver
pub struct X404Resolver {
    cache: SimSimCache,
    generator: DeterministicStubGenerator,
    simulated_saturation: bool,
    simulated_queue_pos: u32,
    simulated_eta: u32,
    active_ticket: Option<String>,
}

impl Default for X404Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl X404Resolver {
    pub fn new() -> Self {
        Self {
            cache: SimSimCache::new(),
            generator: DeterministicStubGenerator::default(),
            simulated_saturation: false,
            simulated_queue_pos: 0,
            simulated_eta: 0,
            active_ticket: None,
        }
    }

    pub fn simulate_saturation(&mut self, saturated: bool, queue_pos: u32, eta: u32) {
        self.simulated_saturation = saturated;
        self.simulated_queue_pos = queue_pos;
        self.simulated_eta = eta;
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Resolve an x404 envelope string (JSON).
    /// Returns JSON serialized ResolverResponse.
    pub fn resolve_json(&mut self, envelope_json: &str) -> String {
        // 1. Parse & validate envelope (G1)
        let envelope = match X404Envelope::parse(envelope_json) {
            Ok(env) => env,
            Err(e) => {
                let resp = ResolverResponse::Error(e.to_string());
                return serde_json::to_string_pretty(&resp).unwrap_or_default();
            }
        };

        let env_hash = envelope.canonical_hash();
        let ticket_id = format!("tkt-{}", &env_hash[0..12]);
        self.active_ticket = Some(ticket_id.clone());

        // 2. Check saturation for x420 backpressure (G6)
        if self.simulated_saturation {
            let x420 = X420Envelope::new(
                &ticket_id,
                self.simulated_queue_pos,
                self.simulated_eta,
            );
            let resp = ResolverResponse::Patience(x420);
            return serde_json::to_string_pretty(&resp).unwrap_or_default();
        }

        // 3. Check SIMSIM content-addressed cache (G3)
        if let Some(cached_result) = self.cache.get(&env_hash) {
            let resp = ResolverResponse::Success(cached_result);
            return serde_json::to_string_pretty(&resp).unwrap_or_default();
        }

        // 4. Trigger local edge generation (G2, G4, G5)
        let result = self.generator.generate(&envelope);

        // 5. Store in content-addressed cache
        self.cache.put(env_hash, result.clone());

        let resp = ResolverResponse::Success(result);
        serde_json::to_string_pretty(&resp).unwrap_or_default()
    }

    /// Handle client feedback/commands in response to x420 backpressure (G6).
    pub fn handle_client_action(&mut self, action_str: &str) -> String {
        let ticket = self.active_ticket.clone().unwrap_or_else(|| "tkt-current".to_string());

        if is_fsck_off_command(action_str) {
            // Polite exit from queue
            self.simulated_saturation = false;
            let receipt = FsckOffReceipt::new(&ticket);
            let resp = ResolverResponse::FsckOff(receipt);
            serde_json::to_string_pretty(&resp).unwrap_or_default()
        } else {
            let resp = ResolverResponse::Error(format!(
                "Action '{}' not processed or requires phase 2 bounty settlement.",
                action_str
            ));
            serde_json::to_string_pretty(&resp).unwrap_or_default()
        }
    }
}

// ---------------------------------------------------------------------------
// DIRECT C-ABI EXPORTS (Zero-dependency vanilla browser WASM instantiation)
// ---------------------------------------------------------------------------

use std::sync::Mutex;

static GLOBAL_RESOLVER: Mutex<Option<X404Resolver>> = Mutex::new(None);

fn with_global_resolver<F, R>(f: F) -> R
where
    F: FnOnce(&mut X404Resolver) -> R,
{
    let mut lock = GLOBAL_RESOLVER.lock().unwrap();
    if lock.is_none() {
        *lock = Some(X404Resolver::new());
    }
    f(lock.as_mut().unwrap())
}

#[no_mangle]
pub extern "C" fn x404_allocate(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn x404_deallocate(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, 0, size);
        }
    }
}

#[no_mangle]
pub extern "C" fn x404_resolve_direct(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);

    let result_json = with_global_resolver(|resolver| resolver.resolve_json(&input_str));

    let mut out_bytes = result_json.into_bytes();
    out_bytes.shrink_to_fit();
    let out_ptr = out_bytes.as_mut_ptr();
    unsafe {
        *out_len = out_bytes.len();
    }
    std::mem::forget(out_bytes);
    out_ptr
}

#[no_mangle]
pub extern "C" fn x404_simulate_saturation(saturated: i32, queue_pos: u32, eta_seconds: u32) {
    with_global_resolver(|resolver| {
        resolver.simulate_saturation(saturated != 0, queue_pos, eta_seconds);
    });
}

#[no_mangle]
pub extern "C" fn x404_handle_action(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let input_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let input_str = String::from_utf8_lossy(input_bytes);

    let result_json = with_global_resolver(|resolver| resolver.handle_client_action(&input_str));

    let mut out_bytes = result_json.into_bytes();
    out_bytes.shrink_to_fit();
    let out_ptr = out_bytes.as_mut_ptr();
    unsafe {
        *out_len = out_bytes.len();
    }
    std::mem::forget(out_bytes);
    out_ptr
}

// ---------------------------------------------------------------------------
// WASM BINDINGS (wasm32-unknown-unknown / Browser Environment)
// ---------------------------------------------------------------------------

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmX404Resolver {
    inner: X404Resolver,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmX404Resolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: X404Resolver::new(),
        }
    }

    /// Primary WASM entrypoint: accepts serialized JSON x404 work order,
    /// returns serialized JSON result or x420 backpressure envelope.
    #[wasm_bindgen]
    pub fn resolve(&mut self, envelope_json: &str) -> String {
        self.inner.resolve_json(envelope_json)
    }

    /// Handles client backpressure actions (e.g. "FSCK_OFF" or "FSCK OFF").
    #[wasm_bindgen]
    pub fn handle_action(&mut self, action: &str) -> String {
        self.inner.handle_client_action(action)
    }

    /// Simulates queue overload to test x420 patience backpressure.
    #[wasm_bindgen]
    pub fn simulate_saturation(&mut self, saturated: bool, queue_pos: u32, eta_seconds: u32) {
        self.inner.simulate_saturation(saturated, queue_pos, eta_seconds);
    }

    #[wasm_bindgen]
    pub fn cache_size(&self) -> usize {
        self.inner.cache_size()
    }

    #[wasm_bindgen]
    pub fn version(&self) -> String {
        "x404-resolver-v0.1.0-edge".to_string()
    }
}

// ---------------------------------------------------------------------------
// UNIT TESTS (Green Acceptance Gates G1, G2, G3, G6)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_valid_envelope_json() -> &'static str {
        r#"{
            "envelope_version": "x404-0.1",
            "uri": "https://hackt.com/liturgy/daily-manifest",
            "description": "Daily liturgical manifest for sovereign builder node",
            "context": {
                "author": "agy@wolfram",
                "mandate": "min-114-x404"
            },
            "policy": {
                "allowed_modalities": ["text/markdown"],
                "max_compute_ms": 10000,
                "local_only": true,
                "grace_attestation": true
            },
            "verification_requirements": {
                "min_content_length": 64,
                "schema_url": null,
                "expected_semantic_tags": ["liturgy", "sovereign"],
                "integrity_hash_algo": "sha256"
            },
            "created_at": "2026-10-23T00:00:00Z",
            "client_identity": "did:key:z6MkhaXgBZDvotDkL5257faiz48Z8xJ8SSkov1QEQ3xzCVnd"
        }"#
    }

    #[test]
    fn test_g1_envelope_parsing_and_validation() {
        let json = sample_valid_envelope_json();
        let env = X404Envelope::parse(json).expect("G1: Valid envelope must parse cleanly");
        assert_eq!(env.envelope_version, "x404-0.1");
        assert_eq!(env.uri, "https://hackt.com/liturgy/daily-manifest");
        assert!(env.policy.local_only);
        assert!(!env.canonical_hash().is_empty());

        // Test invalid envelope (empty URI)
        let invalid_json = r#"{
            "envelope_version": "x404-0.1",
            "uri": "",
            "description": "Missing URI",
            "context": {},
            "policy": {
                "allowed_modalities": ["text/markdown"],
                "max_compute_ms": 10000,
                "local_only": true,
                "grace_attestation": true
            },
            "verification_requirements": {
                "min_content_length": 10,
                "schema_url": null,
                "expected_semantic_tags": [],
                "integrity_hash_algo": "sha256"
            }
        }"#;
        assert!(X404Envelope::parse(invalid_json).is_err());
    }

    #[test]
    fn test_g2_deterministic_stub_generator() {
        let json = sample_valid_envelope_json();
        let env = X404Envelope::parse(json).unwrap();
        let gen = DeterministicStubGenerator::default();

        let res1 = gen.generate(&env);
        let res2 = gen.generate(&env);

        // Must be deterministic and non-trivial
        assert_eq!(res1.content_body, res2.content_body);
        assert_eq!(res1.content_hash, res2.content_hash);
        assert!(res1.content_body.len() >= env.verification_requirements.min_content_length);
        assert_eq!(res1.generator_mode, generator::GeneratorMode::DeterministicStub);
        assert!(res1.content_hash.starts_with("infs:sha256:"));
        assert!(res1.our_lady_grace);
    }

    #[test]
    fn test_g3_content_addressed_deduplication() {
        let mut resolver = X404Resolver::new();
        let json = sample_valid_envelope_json();

        // 1st request: Cache miss -> generation
        let resp1_str = resolver.resolve_json(json);
        let resp1: ResolverResponse = serde_json::from_str(&resp1_str).unwrap();

        let body1 = match resp1 {
            ResolverResponse::Success(res) => {
                assert!(!res.from_cache, "First request must be freshly generated");
                assert_eq!(resolver.cache_size(), 1);
                res.content_body
            }
            _ => panic!("Expected Success"),
        };

        // 2nd request with exact same envelope: Cache hit -> zero re-generation
        let resp2_str = resolver.resolve_json(json);
        let resp2: ResolverResponse = serde_json::from_str(&resp2_str).unwrap();

        match resp2 {
            ResolverResponse::Success(res) => {
                assert!(res.from_cache, "Second request must be returned from cache (G3)");
                assert_eq!(res.content_body, body1);
                assert_eq!(resolver.cache_size(), 1, "Cache size must not duplicate entries");
            }
            _ => panic!("Expected cached Success"),
        }
    }

    #[test]
    fn test_g6_x420_patience_and_fsck_off() {
        let mut resolver = X404Resolver::new();
        let json = sample_valid_envelope_json();

        // Simulate server/solver slot saturation
        resolver.simulate_saturation(true, 7, 30);

        let resp_str = resolver.resolve_json(json);
        let resp: ResolverResponse = serde_json::from_str(&resp_str).unwrap();

        let ticket_id = match resp {
            ResolverResponse::Patience(x420) => {
                assert_eq!(x420.status, 420);
                assert_eq!(x420.queue_position, 7);
                assert_eq!(x420.eta_seconds, 30);
                assert!(x420.options.contains(&"FSCK_OFF".to_string()));
                x420.ticket_id
            }
            _ => panic!("Expected Patience backpressure (x420)"),
        };

        // Client chooses to exit politely via FSCK OFF
        let fsck_str = resolver.handle_client_action("FSCK OFF");
        let fsck_resp: ResolverResponse = serde_json::from_str(&fsck_str).unwrap();

        match fsck_resp {
            ResolverResponse::FsckOff(receipt) => {
                assert_eq!(receipt.status, "ORDER_ABORTED_POLITELY");
                assert_eq!(receipt.command, "FSCK_OFF");
                assert_eq!(receipt.ticket_id, ticket_id);
                assert!(receipt.released_queue_slot);
            }
            _ => panic!("Expected FsckOffReceipt"),
        }
    }
}
