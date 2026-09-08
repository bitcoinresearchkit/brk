"""Independent binary verification; kept separate so JSON parity failures cannot hide it."""
import pytest

from bitcoin_wire import assert_transaction_bytes, decode_block, verify_merkleblock, verify_witness_commitment
from compat import digest

pytestmark = pytest.mark.live


def test_complete_raw_block(target, chain_case):
    _, bh, info, txids = chain_case
    code, _, raw = target.request(f'/api/block/{bh}/raw')
    assert code == 200
    header, parsed = decode_block(raw)
    verify_witness_commitment(parsed)
    assert digest(header)[::-1].hex() == bh
    assert len(raw) == info['size']
    assert [tx['txid'] for tx in parsed] == txids
    overhead = len(raw) - sum(tx['size'] for tx in parsed)
    assert sum(tx['weight'] for tx in parsed) + 4 * overhead == info['weight']


@pytest.mark.parametrize('position', [0, 1, -1], ids=['coinbase', 'first-regular', 'last'])
def test_transaction_wire_fields(target, chain_case, position):
    _, _, _, txids = chain_case
    if position >= len(txids):
        pytest.skip('Fixture block has no regular transaction')
    txid = txids[position]
    raw = bytes.fromhex(target.text(f'/api/tx/{txid}/hex'))
    assert_transaction_bytes(target.json(f'/api/tx/{txid}'), raw)


@pytest.mark.parametrize('position', [0, -1], ids=['first', 'last'])
def test_binary_merkleblock_proof(target, chain_case, position):
    _, bh, _, txids = chain_case
    txid = txids[position]
    proof = bytes.fromhex(target.text(f'/api/tx/{txid}/merkleblock-proof'))
    assert verify_merkleblock(proof, txid, bh) == (position % len(txids))
