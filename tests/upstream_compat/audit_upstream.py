"""Check a local upstream checkout against the pinned route audit. No network I/O.

Usage: python tests/upstream_compat/audit_upstream.py /path/to/mempool
Reports route additions/removals and changed source digests; never edits the suite.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).parent


def extract_routes(source):
    pattern = r"\.(get|post)\(config\.MEMPOOL\.API_URL_PREFIX\s*\+\s*((?:'[^']*'\s*\+\s*)*'[^']*')"
    return {(m[1].upper(), '/api/v1/' + ''.join(re.findall("'([^']*)'", m[2])))
            for m in re.finditer(pattern, source)}


def audit(checkout):
    manifest = json.loads((ROOT/'endpoints.json').read_text())
    inventory = json.loads((ROOT/'upstream_routes.json').read_text())
    findings = []
    for path, digest in manifest['route_sources'].items():
        relative = 'backend/src/api/' + path
        source = (checkout/relative).read_bytes()
        old = {(row['method'], row['route']) for row in inventory if row['source'] == relative}
        current = extract_routes(source.decode())
        for method, route in sorted(current - old):
            findings.append(f'Unclassified route: {method} {route} ({relative})')
        for method, route in sorted(old - current):
            findings.append(f'Removed route: {method} {route} ({relative})')
        if hashlib.sha256(source).hexdigest() != digest:
            findings.append(f'Changed handlers/contracts: {relative}; review against the pinned revision')
    return findings


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mempool_source', type=Path)
    parser.add_argument('--esplora-source', type=Path, help='Optional Esplora checkout to audit API.md')
    args = parser.parse_args()
    findings = audit(args.mempool_source)
    if args.esplora_source:
        manifest = json.loads((ROOT/'endpoints.json').read_text())
        content = (args.esplora_source/'API.md').read_bytes()
        if hashlib.sha256(content).hexdigest() != manifest['esplora_api_sha256']:
            findings.append('Changed Esplora API.md; audit routes and semantics against the pinned revision')
    print('\n'.join(findings) if findings else 'All audited route sources match the pinned contracts.')
    raise SystemExit(bool(findings))
