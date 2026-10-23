# Hack Temple (`hackt.com`) — Sovereign Identity & xHTTP Protocol Suite

Canonical repository for `hackt.com` (Hack Temple), root trust anchor for sovereign identities, and edge-first xHTTP protocol implementations:
- **x404 Generative 404 Protocol & x420 Patience Protocol (`x404-resolver`)**: [`x404-resolver/README.md`](x404-resolver/README.md)
- **Architecture Specification**: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- **14 IRS Criteria & Canon**: [`canon.json`](canon.json)
- **Liturgy & Manifest**: [`index.html`](index.html)

---

## Quick Start: x404 / x420 Edge Resolver

```bash
cd x404-resolver
cargo test --no-default-features
cargo build --target wasm32-unknown-unknown --release
python3 tests/test_x404.py
```
