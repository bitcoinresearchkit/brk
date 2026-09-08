"""Boundary and parameter variants beyond the upstream documentation examples."""
import pytest

from compat import same, shape
from contracts import core_contract
from mempool_contracts import mempool_contract

pytestmark = pytest.mark.live
BLOCK = '000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce'
TX = '15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521'
PERIOD_ROUTES = [
    '/api/v1/mining/pools', '/api/v1/mining/hashrate',
    '/api/v1/mining/hashrate/pools', '/api/v1/mining/difficulty-adjustments',
    '/api/v1/mining/blocks/fees', '/api/v1/mining/blocks/rewards',
    '/api/v1/mining/blocks/fee-rates', '/api/v1/mining/blocks/sizes-weights',
]


@pytest.mark.parametrize('period', ['24h', '3d', '1w', '1m', '3m', '6m', '1y', '2y', '3y'])
@pytest.mark.parametrize('route', PERIOD_ROUTES)
def test_mining_periods(target, references, route, period):
    path = route + '/' + period
    actual = target.json(path)
    mempool_contract(path, actual)
    shape(actual, references['mempool'].json(path, cache=True), path)


@pytest.mark.parametrize('currency', ['USD', 'EUR', 'GBP', 'CAD', 'CHF', 'AUD', 'JPY'])
def test_historical_currencies(target, references, currency):
    path = f'/api/v1/historical-price?currency={currency}&timestamp=1500000000'
    # Exchange sources can revise rates; compare representation, not stale snapshots.
    actual = target.json(path)
    mempool_contract(path, actual)
    shape(actual, references['mempool'].json(path, cache=True), path)


@pytest.mark.parametrize('suffix', ['/txs/1', '/txs/-25', '/txs/abc', '/txid/-1', '/txid/99999999'])
def test_block_pagination_errors(target, references, suffix):
    path = f'/api/block/{BLOCK}{suffix}'
    actual, _, body = target.request(path)
    expected, _, ref_body = references['mempool'].request(path)
    assert expected in (400, 404, 422), f'REFERENCE unexpected {expected}: {ref_body[:200]!r}'
    assert actual == expected, f'{path}: HTTP {actual} != {expected}: {body[:200]!r}'


def test_all_outspends_match_individual_outputs(target, references):
    path = f'/api/tx/{TX}'
    outputs = target.json(path + '/outspends')
    tx = references['mempool'].json(path, cache=True)
    assert len(outputs) == len(tx['vout'])
    core_contract(path + '/outspends', outputs)
    # Old confirmed spends are stable; compare every output, never just item zero.
    expected = references['mempool'].json(path + '/outspends', cache=True)
    for index, value in enumerate(outputs):
        same(target.json(path + f'/outspend/{index}'), value)
        if expected[index]['spent'] and expected[index]['status']['confirmed']:
            same(value, expected[index])


def test_unconfirmed_transaction(target, references):
    reference = references['mempool']
    txids = reference.json('/api/mempool/txids')
    if not txids:
        pytest.skip('Reference mempool empty: no unconfirmed transaction fixture')
    # Node mempools legitimately differ. Select from the intersection only.
    shared = sorted(set(txids).intersection(target.json('/api/mempool/txids')))
    if not shared:
        pytest.skip('No shared unconfirmed transaction; live-node scenario unavailable')
    path = '/api/tx/' + shared[0]
    before = reference.json(path)
    if before['status']['confirmed']:
        pytest.skip('Reference transaction confirmed during discovery')
    actual = target.json(path)
    after = reference.json(path + '/status')
    if after['confirmed'] or actual['status']['confirmed']:
        pytest.skip('Transaction confirmed during live comparison')
    # Transaction bytes, inputs and outputs are immutable even in a changing mempool.
    same(actual, before)
    core_contract(path, actual)
    same(target.json(path + '/status'), after)


@pytest.mark.parametrize('address', ['not-an-address', 'bc1invalid', '1KFHE7w8BhaENAswwryaoccDb6qcT6DbYZ'])
def test_address_validation_false(target, references, address):
    path = '/api/v1/validate-address/' + address
    actual = target.json(path)
    same(actual, references['mempool'].json(path, cache=True))
    assert actual['isvalid'] is False
