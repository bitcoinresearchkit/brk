#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [ -f "$SCRIPT_DIR/.tokens" ]; then
    source "$SCRIPT_DIR/.tokens"
fi

echo "=== Cloudflare cache purge ==="
echo ""

if [ -z "$CF_PURGE_API_TOKEN" ]; then
    echo "CF_PURGE_API_TOKEN not set. Add it to scripts/.tokens"
    exit 1
fi
if [ -z "$CF_ZONE_ID" ]; then
    echo "CF_ZONE_ID not set. Add it to scripts/.tokens"
    exit 1
fi

RESPONSE=$(curl -sS -X POST \
    "https://api.cloudflare.com/client/v4/zones/$CF_ZONE_ID/purge_cache" \
    -H "Authorization: Bearer $CF_PURGE_API_TOKEN" \
    -H "Content-Type: application/json" \
    --data '{"purge_everything":true}')

if echo "$RESPONSE" | grep -q '"success":true'; then
    echo "OK"
else
    echo "Purge failed:"
    echo "$RESPONSE"
    exit 1
fi
