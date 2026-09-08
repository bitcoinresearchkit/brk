"""One independently sourced HTTP case per inventory entry, including absent routes."""
import hashlib
import json

import pytest

from compat import ENDPOINTS, decode_json, hash_id, same, shape, transaction
from contracts import core_contract
from mempool_contracts import mempool_contract

pytestmark = pytest.mark.live



def resolve(endpoint, reference):
    path = endpoint['path']
    address = '1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv'
    if '{tip_height}' in path:
        path = path.replace('{tip_height}', reference.text('/api/blocks/tip/height'))
    if '{block_txid}' in path:
        bh = path.split('/block/', 1)[1].split('/', 1)[0]
        txids = reference.json('/api/block/' + bh + '/txids', cache=True)
        path = path.replace('{block_txid}', txids[min(1, len(txids)-1)])
    if '{scripthash}' in path:
        txs = reference.json(f'/api/address/{address}/txs/chain', cache=True)
        scripts = [v['scriptpubkey'] for tx in txs for v in tx['vout']
                   if v.get('scriptpubkey_address') == address]
        assert scripts, 'REFERENCE DATA: no script for scripthash fixture address'
        script_hash = hashlib.sha256(bytes.fromhex(scripts[0])).digest()[::-1].hex()
        path = path.replace('{scripthash}', script_hash)
    if '{cursor}' in path:
        base = path.rsplit('/', 1)[0]
        page = reference.json(base, cache=True)
        assert page, 'REFERENCE DATA: no transactions for pagination fixture'
        path = path.replace('{cursor}', page[-1]['txid'])
    return path


@pytest.mark.parametrize('endpoint', [e for e in ENDPOINTS if not e.get('exclude') and e['method'] == 'GET'], ids=lambda e: e['id'])
def test_endpoint(target, references, endpoint):
    reference = references[endpoint['source']]
    path = resolve(endpoint, reference)
    actual_status, actual_headers, actual_body = target.request(path)
    allowed = endpoint.get('success_statuses', [200])
    assert actual_status in allowed, f'TARGET {path}: HTTP {actual_status}: {actual_body[:300]!r}'
    ref_status, ref_headers, ref_body = reference.request(path, cache=True)
    if ref_status not in allowed:
        raise RuntimeError(f'REFERENCE UNAVAILABLE {reference.base}{path}: HTTP {ref_status}: {ref_body[:200]!r}')
    if actual_status == 204 or ref_status == 204:
        if actual_status == 204:
            assert actual_body == b''
        else:
            transaction(decode_json(actual_body))
        return
    is_json = 'json' in ref_headers.get('Content-Type', '')
    if is_json:
        assert 'json' in actual_headers.get('Content-Type', ''), f'{path}: expected JSON Content-Type'
        actual, expected = decode_json(actual_body), decode_json(ref_body)
        core_contract(path, actual)
        mempool_contract(path, actual)
        if endpoint['mode'] in ('exact', 'address-page'):
            same(actual, expected, path)
        else:
            shape(actual, expected, path)
    elif endpoint['mode'] == 'exact':
        assert actual_body == ref_body, f'{path}: wire bytes differ'
    elif path.endswith('/tip/hash'):
        hash_id(actual_body.decode().strip())
    elif path.endswith('/tip/height'):
        assert actual_body.decode().strip().isdigit()
    else:
        assert actual_headers.get_content_type() == ref_headers.get_content_type()
        assert actual_body, f'{path}: empty response'


@pytest.mark.parametrize('endpoint', [e for e in ENDPOINTS if not e.get('exclude') and e['method'] == 'POST'], ids=lambda e: e['id'])
@pytest.mark.parametrize('invalid', [b'', b'not-a-transaction', b'00'])
def test_broadcast_rejects_invalid_payload(target, endpoint, invalid):
    body = json.dumps([invalid.decode()]).encode() if endpoint['mode'] == 'package' else invalid
    content_type = 'application/json' if endpoint['mode'] == 'package' else 'text/plain'
    code, _, response = target.request(endpoint['path'], 'POST', body, content_type)
    assert code == 400, f'{endpoint["path"]}: expected invalid-payload HTTP 400, got {code}: {response[:200]!r}'
