# Cloudflare Pages 500 Account Provisioning Bug — Community Escalation

**Linear Ticket:** MIN-185  
**Target Forum:** [Cloudflare Community](https://community.cloudflare.com/)  
**Category:** Developers → Workers & Pages  
**Tags:** `pages`, `workers`, `bug-report`, `provisioning`  
**Target Audience:** Cloudflare Community Staff, Engineers, and MVPs  
**Status:** DRAFT READY FOR SUBMISSION  

---

## 1. Quick Copy-Paste: Post Details for Cloudflare Community

### Title:
```text
[Pages] Empty HTTP 500 on all *.pages.dev URLs (Deployment SUCCESS, Edge Serves Empty 500) — Account b5f059f8e6804b3e9ecfe00eeca9d4ed
```

### Body:

```markdown
Hello Cloudflare Team & Community,

We are experiencing an account-level Cloudflare Pages provisioning bug where every request to our Pages project returns an empty **HTTP/2 500 Internal Server Error** directly from Cloudflare's edge, despite successful deployments via the API.

Because we are on the Free Website plan, human technical support ticketing is unavailable in the dashboard. We are posting here in hopes a Cloudflare Staff member / engineer can inspect the account and trigger an internal re-provisioning of the Pages routing worker.

---

### Account & Project Overview

* **Account ID:** `b5f059f8e6804b3e9ecfe00eeca9d4ed`
* **Plan:** Free Website
* **Pages Project Name:** `hackt-com`
* **Canonical URL (Affected):** `https://hackt-com.pages.dev/`
* **Subpaths (Affected):** `https://hackt-com.pages.dev/robots.txt`
* **Non-existent 404 paths (Affected):** `https://hackt-com.pages.dev/non-existent-test-404` (also returns 500)
* **Custom Domain:** Previously attached `hackt.com` (also returned empty 500; currently diverted to GitHub Pages to prevent production outage, but `hackt-com.pages.dev` remains 100% active and untouched for reproduction).

---

### Observed Behavior vs Expected Behavior

* **Expected Behavior:**
  * Static site deployed via API should serve `index.html` (HTTP 200) on `/`, `robots.txt` on `/robots.txt`, and standard 404 on missing assets.
* **Observed Behavior:**
  * The API deployment completes cleanly with status `SUCCESS`. All static files (11 files, static HTML/CSS/images, no custom functions/workers) are accepted and hashed.
  * **However**, any HTTP/HTTPS request hitting Cloudflare Edge (`*.pages.dev`) immediately returns:
    * Status: `HTTP/2 500`
    * Response body length: `content-length: 0` (empty body)
    * Server: `cloudflare`
    * No Cloudflare standard 5xx branded error page, no 520-524 code — just a raw empty 500.

---

### Live cURL Reproduction & Edge Headers

```bash
$ curl -svI https://hackt-com.pages.dev/
* Connected to hackt-com.pages.dev port 443
* ALPN: server accepted h2
* SSL connection using TLSv1.3 / AEAD-CHACHA20-POLY1305-SHA256
> HEAD / HTTP/2
> Host: hackt-com.pages.dev
> User-Agent: curl/8.7.1
> Accept: */*
> 
< HTTP/2 500 
< alt-svc: h3=":443"; ma=86400
< cf-ray: a4ed7be93e4467ee-SJC
< date: Fri, 23 Oct 2026 02:59:08 GMT
< nel: {"report_to":"cf-nel","success_fraction":0.0,"max_age":604800}
< report-to: {"group":"cf-nel","max_age":604800,"endpoints":[{"url":"https://a.nel.cloudflare.com/report/v4?s=..."}]}
< server: cloudflare
< content-length: 0
```

Even requesting a 404 path produces the same empty 500:
```bash
$ curl -svI https://hackt-com.pages.dev/non-existent-file-404-check
< HTTP/2 500 
< cf-ray: a4ed7e6c1e422712-SJC
< content-length: 0
< server: cloudflare
```

---

### Troubleshooting Steps Already Attempted

1. **Re-deployment:** Triggered clean re-deployments via API with identical successful upload results, but edge serving remains 500.
2. **Project Re-creation:** Deleted and re-created the Pages project under the account — empty 500 persisted.
3. **No Pages Functions:** The project contains purely static assets (`index.html`, `robots.txt`, `favicon.svg`, icons). There is no `functions/` directory, no `_middleware.ts`, and no custom worker bindings attached.
4. **Custom Domain detachment:** Removed custom domain bindings to isolate the issue directly to the native `*.pages.dev` subdomain; behavior is identical.

---

### Diagnosis & Request to Cloudflare Staff

Based on similar reports in the Community, this pattern indicates that the account-level internal Pages routing worker / service binding failed to provision or got wedged in the control plane during project initialization.

Could a Cloudflare Staff member / engineer please check account `b5f059f8e6804b3e9ecfe00eeca9d4ed` and trigger a re-provision of the Pages worker runtime?

Thank you for your assistance!
```

---

## 2. Русскоязычный контекст и инструкция (Mindrock Protocol)

### Контекст инцидента
- **Проблема:** Cloudflare Pages на аккаунте `b5f059f8e6804b3e9ecfe00eeca9d4ed` находится в клинче: API рапортует об успешной загрузке ассетов, но внутренний Pages worker маршрутизации не был поднят / застрял в provisioning bug, из-за чего Cloudflare Edge отдает пустой `HTTP 500` (`content-length: 0`) абсолютно на все запросы.
- **Временное решение:** Домен `hackt.com` переведен на GitHub Pages (LE TLS сертификат активен до 20.01.2027), сайт полностью работоспособен.
- **Целевое состояние:** После реакции стаффа Cloudflare и переподнятия воркера Pages на аккаунте `b5f059f8e6804b3e9ecfe00eeca9d4ed`, проект `https://hackt-com.pages.dev` начнет отдавать 200 OK. После этого DNS `hackt.com` будет переключен обратно на Cloudflare Pages по утвержденной политике хостинга Mindrock.

### Как проверить статус починки:
Запустите скрипт проверки из корня репозитория:
```bash
./scripts/verify-pages-500.sh
```
Если скрипт возвращает `STATUS: RESOLVED (HTTP 200)`, можно инициировать миграцию DNS.
