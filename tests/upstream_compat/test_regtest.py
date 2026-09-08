"""Successful broadcasts using caller-supplied, spendable regtest transactions only."""
import json
from pathlib import Path

import pytest

from compat import hash_id

pytestmark = [pytest.mark.live, pytest.mark.regtest]
REGTEST_GENESIS = '0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206'


@pytest.fixture(scope='module')
def funded_transactions(pytestconfig, target):
    filename = pytestconfig.getoption('--regtest-fixtures')
    if not filename:
        pytest.skip('Successful broadcasts require --regtest-fixtures on a disposable regtest target')
    assert target.text('/api/block-height/0') == REGTEST_GENESIS, 'Refusing to broadcast on a non-regtest chain'
    data = json.loads(Path(filename).read_text())
    assert data['network'] == 'regtest'
    for tx in [data['transaction']] + data['package']:
        hash_id(tx['txid'])
        assert bytes.fromhex(tx['hex'])
    assert len(data['package']) >= 2, 'Supply a parent and child to exercise package acceptance'
    return data


def test_regtest_transaction_broadcast(target, funded_transactions):
    tx = funded_transactions['transaction']
    code, _, body = target.request('/api/tx', 'POST', tx['hex'].encode())
    assert code == 200, f'Broadcast HTTP {code}: {body[:300]!r}'
    assert body.decode().strip() == tx['txid']


def test_regtest_package_broadcast(target, funded_transactions):
    txs = funded_transactions['package']
    code, headers, body = target.request('/api/txs/package', 'POST',
        json.dumps([tx['hex'] for tx in txs]).encode(), 'application/json')
    assert code == 200, f'Package HTTP {code}: {body[:300]!r}'
    assert 'json' in headers.get('Content-Type', '')
    result = json.loads(body)
    assert result['package_msg'] == 'success', result
    returned = {tx['txid'] for tx in result['tx-results'].values()}
    assert returned == {tx['txid'] for tx in txs}
    assert all('error' not in tx for tx in result['tx-results'].values()), result
