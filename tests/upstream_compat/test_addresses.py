"""Address accounting, cursor completeness, and script-type equivalence."""
import hashlib

import pytest

from compat import same, transaction

pytestmark = pytest.mark.live
SCRIPT_TYPES = ['p2pkh', 'p2sh', 'v0_p2wpkh', 'v0_p2wsh', 'v1_p2tr']


@pytest.fixture(scope='session')
def script_addresses(references):
    ref = references['esplora']
    bh = ref.text('/api/block-height/800000', cache=True)
    result = {}
    for start in (0, 25, 50, 75):
        txs = ref.json(f'/api/block/{bh}/txs/{start}', cache=True)
        for tx in txs:
            outputs = tx['vout'] + [v['prevout'] for v in tx['vin'] if v['prevout']]
            for output in outputs:
                kind = output['scriptpubkey_type']
                if kind in SCRIPT_TYPES and output.get('scriptpubkey_address'):
                    result.setdefault(kind, {**output, 'fixture_txid': tx['txid']})
        if len(result) == len(SCRIPT_TYPES):
            break
    assert set(result) == set(SCRIPT_TYPES), f'REFERENCE FIXTURE: missing script types {set(SCRIPT_TYPES)-set(result)}'
    return result


@pytest.fixture(params=SCRIPT_TYPES)
def address_case(request, script_addresses):
    output = script_addresses[request.param]
    return output['scriptpubkey_address'], hashlib.sha256(bytes.fromhex(output['scriptpubkey'])).digest()[::-1].hex()


def test_confirmed_address_accounting(target, references, address_case):
    address, _ = address_case
    path = '/api/address/' + address
    ref = references['esplora']
    before = ref.json(path)['chain_stats']
    actual = target.json(path)
    after = ref.json(path)['chain_stats']
    if before != after:
        pytest.skip('Reference address confirmed history changed during observation')
    assert actual['address'] == address
    same(actual['chain_stats'], before)


def test_address_chain_pages_are_complete(target, references, address_case):
    address, _ = address_case
    base = '/api/address/' + address + '/txs/chain'
    ref = references['esplora']
    first = ref.json(base)
    assert first, 'REFERENCE FIXTURE: funded address unexpectedly has no history'
    # Anchor after a known tx so new tips cannot move the compared pages.
    cursor, seen = first[-1]['txid'], {first[-1]['txid']}
    for _ in range(3):
        path = base + '/' + cursor
        expected, actual = ref.json(path), target.json(path)
        same(actual, expected, path)
        assert not seen.intersection(tx['txid'] for tx in actual)
        for tx in actual:
            transaction(tx)
            assert tx['status']['confirmed']
            relevant = tx['vout'] + [v['prevout'] for v in tx['vin'] if v['prevout']]
            assert any(v.get('scriptpubkey_address') == address for v in relevant)
        seen.update(tx['txid'] for tx in actual)
        if len(expected) < 25:
            break
        cursor = expected[-1]['txid']


def test_scripthash_matches_address_confirmed_history(target, references, address_case):
    address, sh = address_case
    address_path, script_path = '/api/address/' + address, '/api/scripthash/' + sh
    # Compare the same stable page on both namespaces and the independent oracle.
    page = references['esplora'].json(address_path + '/txs/chain')
    assert page
    suffix = '/txs/chain/' + page[-1]['txid']
    expected = references['esplora'].json(address_path + suffix)
    same(target.json(address_path + suffix), expected)
    same(target.json(script_path + suffix), expected)
    info = target.json(script_path)
    assert info['scripthash'] == sh


def test_utxo_values_and_completeness(target, references, address_case):
    address, _ = address_case
    path = '/api/address/' + address
    ref = references['esplora']
    before = ref.json(path)
    target_before = target.json(path)
    actual = target.json(path + '/utxo')
    expected = ref.json(path + '/utxo')
    after = ref.json(path)
    target_after = target.json(path)
    if before != after or target_before != target_after:
        pytest.skip('Address changed during UTXO observation')
    # Independent mempools may reserve different confirmed outputs.
    if before['mempool_stats']['tx_count'] == target_before['mempool_stats']['tx_count'] == 0:
        same(sorted(actual, key=lambda u: (u['txid'], u['vout'])),
             sorted(expected, key=lambda u: (u['txid'], u['vout'])))
        assert sum(u['value'] for u in actual) == (target_before['chain_stats']['funded_txo_sum'] - target_before['chain_stats']['spent_txo_sum'])
    for utxo in actual[:25]:
        tx = ref.json('/api/tx/' + utxo['txid'])
        output = tx['vout'][utxo['vout']]
        assert output['value'] == utxo['value']
        assert output['scriptpubkey_address'] == address


@pytest.mark.parametrize('kind', ['address', 'scripthash'])
def test_empty_history(target, kind):
    # Valid, deterministic, unused identity; unlike invalid addresses this must be 200.
    seed = hashlib.sha256(b'bitview-upstream-compat-empty-address-v1').digest()
    from compat import digest
    payload = b'\x00' + seed[:20]
    encoded = int.from_bytes(payload + digest(payload)[:4], 'big')
    alphabet, key = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz', ''
    while encoded:
        encoded, digit = divmod(encoded, 58)
        key = alphabet[digit] + key
    key = '1' + key if kind == 'address' else seed.hex()
    path = f'/api/{kind}/{key}'
    info = target.json(path)
    assert info[kind] == key
    for stats in ('chain_stats', 'mempool_stats'):
        same(info[stats], dict(tx_count=0, funded_txo_count=0, funded_txo_sum=0, spent_txo_count=0, spent_txo_sum=0))
    for suffix in ('/txs', '/txs/chain', '/txs/mempool', '/utxo'):
        assert target.json(path + suffix) == []


@pytest.mark.parametrize('kind', SCRIPT_TYPES)
def test_script_type_transaction(target, references, script_addresses, kind):
    from bitcoin_wire import assert_transaction_bytes
    txid = script_addresses[kind]['fixture_txid']
    path = '/api/tx/' + txid
    actual = target.json(path)
    same(actual, references['esplora'].json(path, cache=True))
    assert_transaction_bytes(actual, bytes.fromhex(target.text(path + '/hex')))
