#!/usr/bin/env bash
set -euo pipefail

# verify-pages-500.sh — Diagnostic probe for Cloudflare Pages 500 provisioning bug
# MIN-185: Track resolution status of Cloudflare Pages account b5f059f8e6804b3e9ecfe00eeca9d4ed

PAGES_URL="https://hackt-com.pages.dev/"
PROD_URL="https://hackt.com/"

echo "=================================================="
echo " Cloudflare Pages 500 Diagnostic Probe (MIN-185)"
echo " Account ID: b5f059f8e6804b3e9ecfe00eeca9d4ed"
echo " Date: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "=================================================="
echo ""

echo ">>> Probing Production Domain: ${PROD_URL} (GitHub Pages standby)"
PROD_STATUS=$(curl -sI -o /dev/null -w "%{http_code}\n" "${PROD_URL}" || true)
echo "    HTTP Status Code: ${PROD_STATUS}"
if [ "${PROD_STATUS}" = "200" ]; then
    echo "    Result: [HEALTHY] Production domain hackt.com is actively serving 200 OK."
else
    echo "    Result: [WARNING] Production domain returned unexpected status ${PROD_STATUS}."
fi

echo ""
echo ">>> Probing Cloudflare Pages Edge: ${PAGES_URL}"
PAGES_STATUS=$(curl -sI -o /dev/null -w "%{http_code}\n" "${PAGES_URL}" || true)

echo "    HTTP Status Code: ${PAGES_STATUS}"
if [ "${PAGES_STATUS}" = "200" ]; then
    echo "    Result: [RESOLVED] Cloudflare Pages returns HTTP 200 OK!"
    echo "            Provisioning fixed. Ready to proceed with DNS migration."
    echo "=================================================="
    exit 0
elif [ "${PAGES_STATUS}" = "500" ]; then
    echo "    Result: [BUG ACTIVE] Cloudflare Pages still returns empty HTTP 500."
    echo "            Internal Pages worker is not yet provisioned by Cloudflare Staff."
    echo "=================================================="
    exit 1
else
    echo "    Result: [UNEXPECTED STATUS] HTTP ${PAGES_STATUS} received."
    echo "=================================================="
    exit 2
fi
