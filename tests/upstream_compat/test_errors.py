"""Malformed and well-formed missing resources must never become a successful body."""
import pytest

pytestmark = pytest.mark.live
ZERO = '0' * 64

BAD_PATHS = [
    '/api/tx/nope', '/api/tx/nope/hex', '/api/tx/nope/raw',
    '/api/tx/nope/status', '/api/tx/nope/merkle-proof',
    '/api/tx/nope/merkleblock-proof', '/api/tx/nope/outspends',
    '/api/tx/nope/outspend/0', '/api/block/nope',
    '/api/block/nope/header', '/api/block/nope/raw',
    '/api/block/nope/status', '/api/block/nope/txs', '/api/block/nope/txids',
    '/api/block/nope/txid/0', '/api/block-height/-1', '/api/block-height/not-a-height',
    '/api/address/not-an-address', '/api/address/not-an-address/txs',
    '/api/address/not-an-address/txs/chain', '/api/address/not-an-address/txs/mempool',
    '/api/address/not-an-address/utxo', '/api/scripthash/nope',
]
MISSING_PATHS = [f'/api/tx/{ZERO}{suffix}' for suffix in ('', '/status', '/hex', '/raw', '/merkle-proof', '/merkleblock-proof', '/outspends', '/outspend/0')]
MISSING_PATHS += [f'/api/block/{ZERO}{suffix}' for suffix in ('', '/header', '/raw', '/status', '/txids', '/txs', '/txid/0')]


@pytest.mark.parametrize('path', BAD_PATHS + MISSING_PATHS)
def test_error_status_matches_upstream(target, references, path):
    reference = references['esplora' if '/scripthash/' in path else 'mempool']
    expected, _, body = reference.request(path)
    if expected not in (400, 404, 422):
        raise RuntimeError(f'REFERENCE ERROR CONTRACT {path}: HTTP {expected}: {body[:200]!r}')
    actual, _, body = target.request(path)
    assert actual == expected, f'{path}: HTTP {actual} != upstream {expected}: {body[:200]!r}'
