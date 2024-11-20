#!/usr/bin/env bash

DATE=$(date -u '+%Y-%m-%d_%H-%M-%S')
OUTPUT="/assets/pwa/${DATE}"

mkdir ".${OUTPUT}"
cp "./assets/pwa/index.html" ".${OUTPUT}/"

pwa-asset-generator "../assets/dove-orange.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --manifest "./manifest.webmanifest" \
    --favicon \
    --padding "0%" \
    --path-override "${OUTPUT}" \
    --quality "100" \
    --opaque "false"

pwa-asset-generator "../assets/dove-white.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --manifest "./manifest.webmanifest" \
    --icon-only \
    --background "#f26610" \
    --padding "10%" \
    --path-override "${OUTPUT}" \
    --quality "100"

pwa-asset-generator "../assets/logo-stamp-orange.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --splash-only \
    --background "#f26610" \
    --padding "min(30vh, 30vw)" \
    --path-override "${OUTPUT}" \
    --quality "100"
