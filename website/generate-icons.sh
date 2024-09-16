#!/usr/bin/env bash

DATE=$(date -u '+%Y-%m-%d_%H:%M:%S')
OUTPUT="/assets/pwa/${DATE}"

mkdir ".${OUTPUT}"
cp "./assets/pwa/index.html" ".${OUTPUT}/"

pwa-asset-generator "../assets/logo-dove-orange.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --manifest "./manifest.webmanifest" \
    --favicon \
    --padding "0%" \
    --path-override "${OUTPUT}" \
    --quality "100" \
    --opaque "false"

pwa-asset-generator "../assets/logo-dove-light.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --manifest "./manifest.webmanifest" \
    --icon-only \
    --background "#f26610" \
    --padding "5%" \
    --path-override "${OUTPUT}" \
    --quality "100"

pwa-asset-generator "../assets/logo-dove-light.svg" ".${OUTPUT}" \
    --index ".${OUTPUT}/index.html" \
    --splash-only \
    --background "#f26610" \
    --padding "min(35vh, 35vw)" \
    --path-override "${OUTPUT}" \
    --quality "100"

# pwa-asset-generator "../assets/logo-icon.svg" "./assets" \
#     --index "./assets/index.html" \
#     --splash-only \
#     --background "#fffaf6" \
#     --padding "min(40vh, 40vw)" \
#     --path-override "/assets" \
#     --quality "100"

# pwa-asset-generator "../assets/logo-icon.svg" "./assets" \
#     --index "./assets/index.html" \
#     --splash-only \
#     --dark-mode \
#     --background "#12100f" \
#     --padding "min(40vh, 40vw)" \
#     --path-override "/assets" \
#     --quality "100"
