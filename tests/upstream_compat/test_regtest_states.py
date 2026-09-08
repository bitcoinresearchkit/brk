"""Deterministic success, spent/unspent, RBF, CPFP, confirmation and reorg scenarios."""
import json
from decimal import Decimal
import os
from pathlib import Path

import pytest

from bitcoin_wire import assert_transaction_bytes
from compat import same
from regtest import Regtest

pytestmark = [pytest.mark.live, pytest.mark.regtest]


@pytest.fixture(scope='module')
def chain(pytestconfig, target):
    rpc = pytestconfig.getoption('--regtest-rpc')
    if not rpc:
        pytest.skip('State transitions require explicit --regtest-rpc on a disposable wallet/node')
    cookie = pytestconfig.getoption('--regtest-cookie')
    auth = Path(cookie).read_text().strip() if cookie else os.environ.get('COMPAT_REGTEST_AUTH')
    assert auth, 'Provide --regtest-cookie or COMPAT_REGTEST_AUTH=user:password'
    return Regtest(rpc, auth, target, pytestconfig.getoption('--state-timeout'))


def test_regtest_confirmation_spend_and_address_accounting(chain, target):
    address = chain.rpc('getnewaddress')
    _, parent = chain.funded(address)
    chain.send(parent)
    path = '/api/address/' + address
    times = target.json('/api/v1/transaction-times?txId[]=' + parent['txid'] + '&txId[]=' + '0'*64 + '&txId[]=' + parent['txid'])
    assert len(times) == 3 and times[0] == times[2] and times[0] > 0 and times[1] == 0
    pool = chain.rpc('getrawmempool', True)
    stats = target.json('/api/mempool')
    assert stats['count'] == len(pool)
    assert stats['vsize'] == sum(tx['vsize'] for tx in pool.values())
    assert stats['total_fee'] == sum(int(Decimal(str(tx['fees']['base'])) * 100000000) for tx in pool.values())
    same(target.json('/api/tx/' + parent['txid'] + '/status'), {'confirmed': False})
    unconfirmed = target.json(path + '/txs/mempool')
    assert [tx['txid'] for tx in unconfirmed] == [parent['txid']]
    same(target.json(path)['mempool_stats'], dict(tx_count=1, funded_txo_count=1,
         funded_txo_sum=parent['sats'], spent_txo_count=0, spent_txo_sum=0))
    assert target.json('/api/tx/' + parent['txid'] + '/outspend/0')['spent'] is False
    mined = chain.mine()[0]
    tx = target.json('/api/tx/' + parent['txid'])
    assert tx['status']['confirmed'] and tx['status']['block_hash'] == mined
    assert_transaction_bytes(tx, bytes.fromhex(parent['hex']))
    same(target.json(path)['chain_stats'], dict(tx_count=1, funded_txo_count=1,
         funded_txo_sum=parent['sats'], spent_txo_count=0, spent_txo_sum=0))
    assert target.json(path + '/txs/mempool') == []
    utxos = target.json(path + '/utxo')
    assert len(utxos) == 1 and utxos[0]['txid'] == parent['txid'] and utxos[0]['value'] == parent['sats']
    child = chain.signed(chain.output(parent), chain.rpc('getnewaddress'))
    chain.send(child)
    spend = target.json('/api/tx/' + parent['txid'] + '/outspend/0')
    same(spend, dict(spent=True, txid=child['txid'], vin=0, status={'confirmed': False}))
    assert target.json(path + '/utxo') == []
    chain.mine()
    same(target.json(path)['chain_stats'], dict(tx_count=2, funded_txo_count=1,
         funded_txo_sum=parent['sats'], spent_txo_count=1, spent_txo_sum=parent['sats']))
    assert target.json('/api/tx/' + parent['txid'] + '/outspend/0')['status']['confirmed']


def test_regtest_rbf_replacement(chain, target):
    coin, original = chain.funded(fee=1000)
    chain.send(original)
    replacement = chain.signed(coin, original['address'], fee=20000)
    chain.send(replacement)
    assert original['txid'] not in target.json('/api/mempool/txids')
    history = target.json('/api/v1/tx/' + original['txid'] + '/rbf')
    assert history['replacements']['tx']['txid'] == replacement['txid']
    cached = target.json('/api/v1/tx/' + original['txid'] + '/cached')
    assert cached['txid'] == original['txid']
    replacing = target.json('/api/v1/tx/' + replacement['txid'] + '/rbf')
    assert original['txid'] in replacing['replaces']
    trees = target.json('/api/v1/replacements')
    assert any(tree['tx']['txid'] == replacement['txid'] for tree in trees)
    chain.mine()


def test_regtest_cpfp_graph(chain, target):
    _, parent = chain.funded(fee=1000)
    chain.send(parent)
    child = chain.signed(chain.output(parent), chain.rpc('getnewaddress'), fee=20000)
    chain.send(child)
    parent_info = target.json('/api/v1/cpfp/' + parent['txid'])
    child_info = target.json('/api/v1/cpfp/' + child['txid'])
    assert parent['txid'] in {a['txid'] for a in child_info['ancestors']}
    descendants = parent_info.get('descendants', [])
    if parent_info.get('bestDescendant'):
        descendants += [parent_info['bestDescendant']]
    assert child['txid'] in {d['txid'] for d in descendants}
    parent_tx = target.json('/api/tx/' + parent['txid'])
    assert parent_info['effectiveFeePerVsize'] > parent_tx['fee'] / (parent_tx['weight'] / 4)
    chain.mine()


def test_regtest_reorg_and_reconfirmation(chain, target):
    _, tx = chain.funded()
    chain.send(tx)
    old_hash = chain.mine()[0]
    try:
        chain.rpc('invalidateblock', old_hash)
        chain.synced()
        orphan = target.json('/api/block/' + old_hash + '/status')
        assert orphan['in_best_chain'] is False
        state = target.json('/api/tx/' + tx['txid'] + '/status')
        assert state['confirmed'] is False
        # A new coinbase destination makes this a distinct competing block.
        new_hash = chain.mine(2)[0]
        assert new_hash != old_hash
        state = target.json('/api/tx/' + tx['txid'] + '/status')
        assert state['confirmed'] and state['block_hash'] == new_hash
        proof = target.json('/api/tx/' + tx['txid'] + '/merkle-proof')
        assert proof['block_height'] == state['block_height']
    finally:
        chain.rpc('reconsiderblock', old_hash)


def test_regtest_generated_package(chain, target):
    _, parent = chain.funded()
    # Wallet knows the parent for signing, but it must not enter the mempool yet.
    decoded = chain.rpc('decoderawtransaction', parent['hex'])
    child_raw = chain.rpc('createrawtransaction', [dict(txid=parent['txid'], vout=0)],
                         {chain.rpc('getnewaddress'): (parent['sats'] - 20000) / 100000000})
    signed = chain.rpc('signrawtransactionwithwallet', child_raw,
        [dict(txid=parent['txid'], vout=0, scriptPubKey=decoded['vout'][0]['scriptPubKey']['hex'], amount=parent['sats']/100000000)])
    assert signed['complete']
    child_id = chain.rpc('decoderawtransaction', signed['hex'])['txid']
    code, _, body = target.request('/api/txs/package', 'POST',
        json.dumps([parent['hex'], signed['hex']]).encode(), 'application/json')
    assert code == 200, f'Package HTTP {code}: {body[:300]!r}'
    result = json.loads(body)
    assert result['package_msg'] == 'success'
    assert {tx['txid'] for tx in result['tx-results'].values()} == {parent['txid'], child_id}
    chain.until(lambda: {parent['txid'], child_id}.issubset(target.json('/api/mempool/txids')), 'accepted package')
    chain.mine()


@pytest.mark.parametrize('path', ['/api/tx', '/api/v1/tx', '/api/v1/tx/push'])
def test_regtest_broadcast_aliases(chain, target, path):
    _, tx = chain.funded()
    is_form = path.endswith('/push')
    body = ('txHash=' + tx['hex']).encode() if is_form else tx['hex'].upper().encode()
    kind = 'text/plain'
    code, _, response = target.request(path, 'POST', body, kind)
    assert code == 200 and response.decode().strip() == tx['txid']
    chain.until(lambda: tx['txid'] in target.json('/api/mempool/txids'), 'broadcast through ' + path)
    chain.mine()
