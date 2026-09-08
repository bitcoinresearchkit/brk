"""Known chain bytes and deliberate corruption prove validators reject bad responses."""
import json
from pathlib import Path

import pytest

from bitcoin_wire import Reader, assert_transaction_bytes, compact, decode_block, decode_transaction, verify_merkleblock
from compat import same
from mempool_contracts import mempool_contract

GENESIS = json.loads(Path(__file__).with_name('fixtures').joinpath('genesis.json').read_text())
RAW = bytes.fromhex(GENESIS['raw'])
PROOF = RAW[:80] + (1).to_bytes(4, 'little') + b'\x01' + bytes.fromhex(GENESIS['txid'])[::-1] + b'\x01\x01'


def test_known_genesis_block_and_transaction():
    header, txs = decode_block(RAW)
    assert len(RAW) == 285 and len(txs) == 1
    tx = txs[0]
    assert tx['txid'] == tx['wtxid'] == GENESIS['txid']
    assert tx['size'] == 204 and tx['weight'] == 816
    assert tx['vin'][0]['vout'] == 0xffffffff
    assert tx['vout'][0]['value'] == 5000000000
    assert verify_merkleblock(PROOF, GENESIS['txid'], GENESIS['block_hash']) == 0


@pytest.mark.parametrize('value', [0, 252, 253, 65535, 65536, 4294967295, 4294967296])
def test_compact_size_boundaries(value):
    reader = Reader(compact(value))
    assert reader.compact() == value
    reader.end()


@pytest.mark.parametrize('encoded', [b'\xfd\xfc\x00', b'\xfe\xff\xff\x00\x00', b'\xff'+bytes(8), b'\xfd\x01'])
def test_bad_compact_size_is_rejected(encoded):
    with pytest.raises(AssertionError):
        Reader(encoded).compact()


@pytest.mark.parametrize('raw', [RAW[:-1], RAW+b'\x00', RAW[:80]+b'\x00', RAW[:80]+b'\x02'+RAW[81:]])
def test_corrupt_raw_block_is_rejected(raw):
    with pytest.raises(AssertionError):
        decode_block(raw)


@pytest.mark.parametrize('proof', [PROOF[:-1], PROOF+b'\x00', PROOF[:80]+bytes(4)+PROOF[84:],
                                  PROOF[:-1]+b'\x00', PROOF[:-1]+b'\x81',
                                  PROOF[:85]+bytes(32)+PROOF[117:]])
def test_corrupt_merkleblock_is_rejected(proof):
    with pytest.raises(AssertionError):
        verify_merkleblock(proof, GENESIS['txid'], GENESIS['block_hash'])


def witness_fixture():
    # Two witness stack elements, including an empty element: removing it must fail.
    return (bytes.fromhex('02000000000101') + bytes.fromhex('11'*32) + bytes(4) +
            bytes.fromhex('00fdffffff01') + (1000).to_bytes(8, 'little') +
            bytes.fromhex('0151020001aa') + bytes(4))


def test_witness_hash_and_weight_accounting():
    raw = witness_fixture()
    tx = decode_transaction(raw)
    assert tx['txid'] != tx['wtxid']
    assert tx['vin'][0]['witness'] == ['', 'aa']
    assert tx['size'] == len(raw) and tx['weight'] == 4 * len(raw) - 3 * 6
    assert_transaction_bytes(tx, raw)


@pytest.mark.parametrize('field', ['weight', 'size', 'version', 'locktime', 'txid', 'vout', 'witness', 'value', 'script'])
def test_wire_json_mutations_fail(field):
    raw = witness_fixture()
    tx = decode_transaction(raw)
    if field in ('weight', 'size', 'version', 'locktime'):
        tx[field] += 1
    elif field == 'txid':
        tx[field] = '0'*64
    elif field == 'vout':
        tx['vin'][0]['vout'] = 65535
    elif field == 'witness':
        tx['vin'][0]['witness'] = ['aa']
    elif field == 'value':
        tx['vout'][0]['value'] += 1
    else:
        tx['vout'][0]['scriptpubkey'] = '00'
    with pytest.raises(AssertionError):
        assert_transaction_bytes(tx, raw)


@pytest.mark.parametrize('path,value', [
    ('/api/v1/mining/blocks/fees/24h', []),
    ('/api/v1/mining/blocks/rewards/24h', None),
    ('/api/v1/cpfp/'+'1'*64, []),
    ('/api/v1/cpfp/'+'1'*64, {'ancestors': [{'txid': '1'*64, 'weight': -1, 'fee': 1}]}),
    ('/api/v1/mining/reward-stats/2', {'startBlock': 10, 'endBlock': 10, 'totalReward': '2', 'totalFee': '1', 'totalTx': '1'}),
    ('/api/v1/transaction-times?txId[]=a&txId[]=b', [1]),
    ('/api/v1/transaction-times?txId[]=a', [True]),
])
def test_plausible_but_wrong_dynamic_responses_fail(path, value):
    with pytest.raises((AssertionError, KeyError, TypeError)):
        mempool_contract(path, value)


def test_registered_route_audit_has_executable_cases():
    from compat import ENDPOINTS
    cases = {e['id']: e for e in ENDPOINTS}
    routes = json.loads(Path(__file__).with_name('upstream_routes.json').read_text())
    for route in routes:
        if route.get('exclude'):
            assert '/internal/' in route['route'] or any(s in route['route'] for s in ('/acceleration', '/services/', '/donations', '/contributors', '/translators'))
        else:
            entry = cases[route['endpoint']]
            assert entry['method'] == route['method'] and not entry.get('exclude')


def test_witness_commitment_detects_equal_length_witness_corruption():
    from bitcoin_wire import verify_witness_commitment
    from compat import digest
    child = decode_transaction(witness_fixture())
    reserved = bytes(32)
    root = digest(bytes(32) + bytes.fromhex(child['wtxid'])[::-1])
    commitment = digest(root + reserved)
    coinbase = dict(size=100, weight=300, vin=[{'witness': [reserved.hex()]}],
                    vout=[{'scriptpubkey': '6a24aa21a9ed' + commitment.hex()}])
    verify_witness_commitment([coinbase, child])
    child['wtxid'] = '00'*32
    with pytest.raises(AssertionError, match='witness commitment'):
        verify_witness_commitment([coinbase, child])


@pytest.mark.parametrize('body', ['{"fee": NaN}', '{"fee": Infinity}', '{"extra": -Infinity}'])
def test_nonstandard_json_numbers_are_rejected(body):
    from compat import decode_json
    with pytest.raises(ValueError):
        decode_json(body)


def test_every_documented_esplora_bitcoin_route_has_a_case():
    from compat import ENDPOINTS
    cases = {e['id']: e for e in ENDPOINTS}
    routes = json.loads(Path(__file__).with_name('esplora_routes.json').read_text())
    assert len({(r['method'], r['route']) for r in routes}) == len(routes)
    for route in routes:
        entry = cases[route['endpoint']]
        assert not entry.get('exclude') and entry['method'] == route['method']
