# Hack Temple (hackt.com)

Official website and digital chapel of Hack Temple.

## Hosting Architecture & Active Incident

* **Production Domain:** [hackt.com](https://hackt.com)
* **Current Hosting:** GitHub Pages (active standby with Let's Encrypt TLS certificate valid until 2027-01-20).
* **Target Hosting:** Cloudflare Pages (`hackt-com.pages.dev`).
* **Active Incident (Linear MIN-185):** Cloudflare Pages currently returns an empty HTTP 500 across all URLs (`*.pages.dev`) due to an account-level provisioning defect on Cloudflare's edge control plane (`Account ID: b5f059f8e6804b3e9ecfe00eeca9d4ed`).

### Diagnostics & Community Escalation

* **Community Post Draft:** [docs/cloudflare-community-post.md](docs/cloudflare-community-post.md) contains the escalation post prepared for the Cloudflare Developers Community forum.
* **Diagnostic Verification Probe:**
  ```bash
  ./scripts/verify-pages-500.sh
  ```
  Returns exit code `0` when Cloudflare Pages is restored (HTTP 200), or exit code `1` while the provisioning bug is active.
