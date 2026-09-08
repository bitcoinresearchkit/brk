"""Query cardinality, invalid cursors and boundary inputs from upstream handlers."""
import json

import pytest

from compat import same

pytestmark = pytest.mark.live
TX = '15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521'
ADDRESS = '1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv'


@pytest.mark.parametrize('query', ['', '?txids=', '?txids=nope', '?txids='+','.join([TX]*51)])
def test_batched_outspend_validation(target, query):
    code, _, body = target.request('/api/v1/txs/outspends'+query)
    assert code == 400, f'Expected upstream validation HTTP 400, got {code}: {body[:200]!r}'


@pytest.mark.parametrize('query', ['', '?txId[]='+TX, '?txId[]='+TX+'&txId[]='+TX, '?txId[]=nope'])
def test_transaction_times_parameters(target, references, query):
    path = '/api/v1/transaction-times'+query
    code, _, body = target.request(path)
    expected, _, ref = references['mempool'].request(path)
    assert expected in (200, 400), f'REFERENCE unavailable: {expected}'
    assert code == expected
    if code == 200:
        same(json.loads(body), json.loads(ref))


@pytest.mark.parametrize('cursor', ['nope', 'g'*64, '0'*64])
def test_address_cursor_validation(target, references, cursor):
    path = f'/api/address/{ADDRESS}/txs/chain/{cursor}'
    code, _, body = target.request(path)
    expected, _, ref = references['esplora'].request(path)
    assert expected in (200, 400, 404), f'REFERENCE unavailable: {expected}'
    assert code == expected
    if code == 200:
        same(json.loads(body), json.loads(ref))


@pytest.mark.parametrize('body', [b'null', b'{}', b'[]', b'[null]', b'[1]', b'{', b'["00","00"]'])
@pytest.mark.parametrize('path', ['/api/txs/package', '/api/v1/txs/package'])
def test_invalid_package_structure(target, path, body):
    code, _, response = target.request(path, 'POST', body, 'application/json')
    assert code == 400, f'{path}: HTTP {code}: {response[:200]!r}'
