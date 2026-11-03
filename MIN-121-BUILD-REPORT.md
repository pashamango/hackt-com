# MIN-121 build receipt — x404 edge resolver

## Outcome

The MIN-114 x404 worktree contains the Rust/WASM edge-first resolver, browser demo,
content-addressed cache, x420 backpressure path, and evidence bundle. The local
Python verification suite passes 6/6. This handoff is not a final release claim:
the current worker image has neither `cargo`/`rustup` nor an available browser
automation surface, so fresh Rust and browser gates remain blocked.

## Evidence / Artifacts

- Worktree: `worktrees/MIN-114-X404-RESOLVER-0001`
- Branch: `minutta/min-114-handoff-tochka-prodolzheniya-na-2026-10-19-vecher-flot`
- Existing HEAD: `e0de875` (`fix(x404): make semantic deduplication canonical`)
- Resolver crate: `x404-resolver/`
- WASM artifacts: `x404-resolver/pkg/x404_resolver.wasm`, `x404-resolver/demo/x404_resolver.wasm`
- Browser demo: `x404-resolver/demo/index.html`, `x404-resolver/demo/app.js`
- Existing evidence: `x404-resolver/evidence/`

## Checks

- PASS (fresh): `python3 tests/test_x404.py` — 6 tests, exit 0.
- PASS (fresh): WASM header is `00 61 73 6d` and artifact is present.
- INHERITED: `x404-resolver/evidence/test_suite.log` records prior Rust/Python evidence.
- BLOCKED (fresh): `cargo test --no-default-features`; `cargo` is absent from PATH.
- BLOCKED (fresh): `cargo build --target wasm32-unknown-unknown --release`; Rust toolchain is absent.
- BLOCKED (fresh): browser execution and network-audit UI; browser access host is unavailable.

## Limitations / next valid command

Run in an environment with Rust 1.80+, `wasm32-unknown-unknown`, and a browser:

```sh
cd x404-resolver
cargo test --no-default-features
cargo build --target wasm32-unknown-unknown --release
python3 tests/test_x404.py
python3 -m http.server 8080 --directory demo
```

After those checks, QA should independently inspect the browser demo's duplicate
request (`from_cache: true`) and zero-network audit before issuing PASS.
