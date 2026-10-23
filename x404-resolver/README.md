# x404 Edge-First Generative Resolver & x420 Patience Protocol (`x404-resolver`)

**Linear Ticket:** `MIN-114`  
**Initiative:** xHTTP / Project x404  
**Builder:** `agy@wolfram`  
**License:** MIT

---

## 1. Overview

`x404-resolver` is a client-side WebAssembly (WASM) and Rust implementation of the **x404 generative 404 protocol** and **x420 patience protocol**.

In traditional web architectures, an unresolved URL returns a dead-end `404 Not Found`. Under x404, an unresolved URI is converted into a **structured work order envelope** that triggers local generation on the user's machine (WASM in browser or native plugin). Zero central servers are involved.

By Directive 2026-10-20 (Principal Pavel Cherkashin), the crate implements **x420 (Enhance Your Calm)**: when edge solver capacity is saturated (WIP=1), the resolver yields an x420 backpressure envelope containing queue position, ETA, and options (`WAIT`, `PAY`, `DELEGATE`, `OUR_LADY`, `FSCK_OFF`). A client sending `FSCK OFF` ("File System Check Off") cleanly cancels its place in queue without error or penalty.

---

## 2. Directory Structure

```
x404-resolver/
├── Cargo.toml               # Manifest (cdylib & rlib)
├── Cargo.lock               # Pinned dependency lockfile
├── ARCHITECTURE.md          # Architectural spec & protocol envelope definitions
├── README.md                # Documentation & usage guide
├── src/
│   ├── lib.rs               # X404Resolver, WASM exports (WasmX404Resolver), C-ABI, unit tests
│   ├── envelope.rs          # X404Envelope parser & validator (G1)
│   ├── generator.rs         # DeterministicStubGenerator (G2)
│   ├── cache.rs             # SimSimCache content-addressed deduplication (G3)
│   └── patience.rs          # x420 Patience envelope & FSCK OFF receipt (G6)
├── tests/
│   └── test_x404.py         # Comprehensive Python verification suite (G1–G7)
├── demo/
│   ├── index.html           # Interactive browser terminal & gate verification UI
│   ├── app.js               # WebAssembly loader and network audit interceptor
│   └── x404_resolver.wasm   # Standalone compiled WebAssembly binary (240 KB)
└── pkg/
    └── x404_resolver.wasm   # Packaged binary artifact
```

---

## 3. Acceptance Gates (G1 – G7) Verification

| Gate | Criterion | Status | Evidence |
|---|---|---|---|
| **G1** | Rust Crate & Envelope Parsing | ✅ PASS | `src/envelope.rs`: parses version, URI, context, policy, and requirements. |
| **G2** | WASM Target (`wasm32-unknown-unknown`) | ✅ PASS | Compiles cleanly to `target/wasm32-unknown-unknown/release/x404_resolver.wasm` (240 KB). |
| **G3** | Content-Addressed Deduplication | ✅ PASS | `src/cache.rs`: canonical hash lookup returns `from_cache: true`; 0 duplicate generation loops. |
| **G4** | Edge-First Proof (Zero Central Server) | ✅ PASS | Core loop executes entirely client-side inside WebAssembly memory. |
| **G5** | "Our Lady" UX | ✅ PASS | Generation is free grace; network socket interceptor confirms 0 outbound network calls. |
| **G6** | x420 & FSCK OFF | ✅ PASS | Saturated slot yields 420; `FSCK OFF` command returns `ORDER_ABORTED_POLITELY`. |
| **G7** | Evidence Bundle | ✅ PASS | `test_x404.py`, `demo/index.html`, logs, and `ARCHITECTURE.md`. |

---

## 4. How to Build & Test

### Prerequisites
- Rust 1.80+ with `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Run Rust Unit Tests
```bash
cargo test --no-default-features
```

### Build WebAssembly Release
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Python Verification Suite
```bash
python3 tests/test_x404.py
```

### Run Interactive Browser Demo
```bash
cd demo
python3 -m http.server 8080
# Open http://localhost:8080 in your browser
```
In the browser demo:
1. Click **⚡ Run Automated Verification (G1–G6)** to run all acceptance gates inside the browser.
2. Observe the **🌐 Network Call Audit** showing 0 external network requests during generation.
3. Test individual buttons: **Resolve x404 Envelope**, **Request Duplicate (G3 Deduplication)**, **Trigger Saturated (x420)**, and **Issue Client FSCK OFF**.
