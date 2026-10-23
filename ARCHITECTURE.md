# Hack Temple: x404 Edge Resolver & x420 Patience Protocol Architecture

**Canonical Specification:** `ARCHITECTURE.md`  
**Status:** Canonical / Active Implementation (MIN-114)  
**Version:** 0.2.0 (2026-10-23)  
**Initiative:** xHTTP / Project x404  
**Author:** `agy@wolfram` (Under Directive from Principal Pavel Cherkashin)

---

## 1. Executive Summary

Traditional web architecture treats `404 Not Found` as an epistemic dead end. In sovereign computing, an unresolved URI is not an error—it is an **executable work order**.

The **x404 Generative Protocol** transforms dead links into content-addressed generation requests executed locally on the user's edge machine (via WebAssembly in the browser or local client plugins). Centralized compute servers are bypassed entirely by design.

By Directive 2026-10-20 (Pavel Cherkashin), the architecture integrates **x420 (Protocol of Patience)**: a machine-readable backpressure mechanism based on `HTTP 420 Enhance Your Calm`. When local resources or solver slots saturate (WIP=1), the resolver yields a structured x420 envelope offering explicit options (`WAIT`, `PAY`, `DELEGATE`, `OUR_LADY`, and `FSCK_OFF`), allowing the client to politely abort the queue (`FSCK OFF`) without penalty.

```
+-----------------------------------------------------------------------------------+
| USER BROWSER / EDGE RUNTIME                                                       |
|                                                                                   |
|  Unresolved URI  ──> [ x404 Work Order Envelope ]                                 |
|                                │                                                  |
|                                ▼                                                  |
|                   [ WasmX404Resolver (WASM) ]                                     |
|                                │                                                  |
|         ┌──────────────────────┴──────────────────────┐                           |
|         ▼                                             ▼                           |
|  Slot Saturated?                              Slot Available                      |
|         │                                             │                           |
|         ▼                                             ▼                           |
|  [ x420 Patience Envelope ]               [ SIMSIM Content Cache ]                |
|  (WAIT / DELEGATE / FSCK OFF)                         │                           |
|         │                                  ┌──────────┴──────────┐                |
|         ▼                                  ▼                     ▼                |
|   Client: "FSCK OFF"                   Cache Hit?            Cache Miss?          |
|         │                                  │                     │                |
|         ▼                                  ▼                     ▼                |
|  [ FsckOffReceipt: Clean Exit ]    [ Return Artifact ]   [ Local Edge Gen ]       |
|                                    (Zero Regens)                 │                |
|                                                                  ▼                |
|                                                        [ Store in INFS Cache ]    |
|                                                                  │                |
|                                                                  ▼                |
|                                                        [ Render "Our Lady" UX ]   |
|                                                        (0 Outbound Network Calls) |
+-----------------------------------------------------------------------------------+
```

---

## 2. Core Protocol Envelopes

### A. The x404 Work Order Envelope
When an agent or user traverses to a non-existent path on `hackt.com` or any sovereign namespace:

```json
{
  "envelope_version": "x404-0.1",
  "uri": "https://hackt.com/liturgy/daily-manifest-2026",
  "description": "Daily liturgical manifest for sovereign builder node",
  "context": {
    "node": "wolfram",
    "agent": "agy@wolfram"
  },
  "policy": {
    "allowed_modalities": ["text/markdown", "application/json"],
    "max_compute_ms": 15000,
    "local_only": true,
    "grace_attestation": true
  },
  "verification_requirements": {
    "min_content_length": 64,
    "schema_url": null,
    "expected_semantic_tags": ["liturgy", "sovereign", "edge"],
    "integrity_hash_algo": "sha256"
  },
  "created_at": "2026-10-23T00:00:00Z",
  "client_identity": "did:key:z6MkhaXgBZDvotDkL5257faiz48Z8xJ8SSkov1QEQ3xzCVnd"
}
```

### B. The x420 Patience Envelope (Backpressure)
When the local edge slot is saturated (queue backlog > 0):

```json
{
  "status": 420,
  "status_text": "420 Enhance Your Calm — Patience Protocol Active",
  "ticket_id": "tkt-7fa8b12e3456",
  "queue_position": 3,
  "eta_seconds": 15,
  "retry_after_ms": 15000,
  "options": ["WAIT", "PAY", "DELEGATE", "OUR_LADY", "FSCK_OFF"],
  "timestamp": "2026-10-23T00:00:00Z",
  "backpressure_notice": "Local edge solver slot is saturated (WIP=1). Please choose a patience option or gracefully abort via FSCK OFF."
}
```

### C. Client Option Semantics
1. **`WAIT`**: Client polls after `retry_after_ms` with exponential backoff.
2. **`PAY`**: Micro-bounty prioritization via Grace token (Phase 2, specified in `GRACE-CURRENCY.md`).
3. **`DELEGATE`**: Handoff to peer agent in Tailscale mesh (`chitakasha`).
4. **`OUR_LADY`**: Request free priority slot under sovereign mercy.
5. **`FSCK_OFF`**: ("File System Check Off") — Polite refusal to wait. The client cleanly revokes its spot in the queue. The solver immediately marks the order aborted and returns an `FsckOffReceipt`:

```json
{
  "status": "ORDER_ABORTED_POLITELY",
  "ticket_id": "tkt-7fa8b12e3456",
  "command": "FSCK_OFF",
  "message": "File system check off: Queue slot released cleanly with zero client penalty. In You We Trust.",
  "released_queue_slot": true,
  "timestamp": "2026-10-23T00:00:00Z"
}
```

---

## 3. Invariants & Acceptance Gates

| Gate | Name | Architectural Invariant | Verification Mechanism |
|---|---|---|---|
| **G1** | Rust Envelope Parser | Strict parsing of description, context, policy, requirements. Rejects non-local policies. | `cargo test test_g1_envelope_parsing_and_validation` |
| **G2** | WASM Target | Compiles clean to `wasm32-unknown-unknown` and executes in browser runtime. | `x404_resolver.wasm` validated binary; `test_g2_wasm_target_exists_and_valid` |
| **G3** | Content-Addressed Dedup | Canonical SHA-256 over normalized envelope; identical request hits SIMSIM cache without loop. | `test_g3_content_addressed_deduplication` (asserts `from_cache: true`) |
| **G4** | Edge-First Execution | Complete core generation loop executes client-side without central server or remote APIs. | Isolated browser test, socket connection interceptor |
| **G5** | "Our Lady" UX | Generation appears as unconditional grace. Private keys and telemetry never leave machine. | `test_g4_g5_zero_network_audit` (0 network calls verified) |
| **G6** | x420 & FSCK OFF | Saturated slot emits structured 420; client `FSCK OFF` cleanly cancels queue slot. | `test_g6_x420_patience_and_fsck_off` |
| **G7** | Evidence Bundle | Complete manifest, test logs, browser demo, and verified Git coordinates. | `evidence/MANIFEST.sha256`, `evidence/EVIDENCE.json` |

---

## 4. Content Addressing & Infinite Storage (SIMSIM / INFS)

Every generated artifact receives an immutable INFS identifier derived from the SHA-256 digest of its synthesized body:

$$\text{content\_hash} = \text{"infs:sha256:"} + \text{SHA256}(\text{content\_body})$$

The deduplication cache stores a mapping:

$$\text{canonical\_envelope\_hash} \longmapsto \text{CachedArtifact}(\text{result}, \text{hits}, \text{timestamp})$$

Any duplicate navigation to the same unresolved URI immediately resolves from client-side memory in $< 1\text{ ms}$, mathematically preventing regeneration recursion loops.

---

## 5. Security & Isolation Discipline

1. **Zero-Trust Sandboxing**: The resolver executes in WebAssembly memory isolation. It possesses no capabilities to access host filesystem or networks unless explicitly piped through host bindings.
2. **Key Protection**: Client DID (`did:key`) remains strictly confined to edge browser storage (`localStorage` or in-memory session). No private keys or prompt telemetry are ever dispatched to external telemetry endpoints.
3. **Grace Model (Our Lady)**: Compute is presented as an unencumbered sovereign blessing. No sign-in wall, no forced tracking.
