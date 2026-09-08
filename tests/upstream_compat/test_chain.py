"""Multi-era immutable parity and independent Bitcoin proof/pagination checks."""
import pytest

from compat import block, digest, same, transaction

pytestmark = pytest.mark.live


def test_block_header_and_merkle_root(target, chain_case):
    ref, bh, expected, txids = chain_case
    actual = target.json(f'/api/block/{bh}')
    same(actual, expected)
    block(actual)
    header = bytes.fromhex(target.text(f'/api/block/{bh}/header'))
    assert len(header) == 80 and digest(header)[::-1].hex() == bh
    assert header[36:68][::-1].hex() == actual['merkle_root']
    assert int.from_bytes(header[68:72], 'little') == actual['timestamp']
    assert int.from_bytes(header[72:76], 'little') == actual['bits']
    assert int.from_bytes(header[76:80], 'little') == actual['nonce']
    got = target.json(f'/api/block/{bh}/txids')
    same(got, txids)
    assert len(got) == actual['tx_count']
    nodes = [bytes.fromhex(txid)[::-1] for txid in got]
    while len(nodes) > 1:
        if len(nodes) % 2:
            nodes.append(nodes[-1])
        nodes = [digest(nodes[i] + nodes[i+1]) for i in range(0, len(nodes), 2)]
    assert nodes[0] == header[36:68]


@pytest.mark.parametrize('position', [0, 1], ids=['coinbase', 'regular'])
def test_transaction_and_proof(target, chain_case, position):
    ref, bh, info, txids = chain_case
    if position >= len(txids):
        pytest.skip('Fixture block has no non-coinbase transaction')
    txid = txids[position]
    path = f'/api/tx/{txid}'
    actual = target.json(path)
    same(actual, ref.json(path, cache=True))
    transaction(actual)
    proof = target.json(path + '/merkle-proof')
    same(proof, ref.json(path + '/merkle-proof', cache=True))
    assert proof['pos'] == position and proof['block_height'] == info['height']
    node = bytes.fromhex(txid)[::-1]
    index = proof['pos']
    for sibling in proof['merkle']:
        sibling = bytes.fromhex(sibling)[::-1]
        node = digest(sibling + node if index & 1 else node + sibling)
        index >>= 1
    assert node[::-1].hex() == info['merkle_root'] and index == 0
    raw_status, _, raw = target.request(path + '/raw')
    assert raw_status == 200
    assert raw.hex() == target.text(path + '/hex')
    assert len(raw) == actual['size']
    assert raw == bytes.fromhex(ref.text(path + '/hex', cache=True))
    same(target.json(path + '/status'), ref.json(path + '/status', cache=True))


def test_block_pagination(target, chain_case):
    ref, bh, info, txids = chain_case
    # First, second and last pages cover boundaries without thousands of requests.
    starts = sorted({0, min(25, (len(txids)-1)//25*25), (len(txids)-1)//25*25})
    for start in starts:
        path = f'/api/block/{bh}/txs/{start}'
        actual = target.json(path)
        same(actual, ref.json(path, cache=True))
        assert [tx['txid'] for tx in actual] == txids[start:start+25]
        for tx in actual:
            transaction(tx)
    assert target.text(f'/api/block/{bh}/txid/{len(txids)-1}') == txids[-1]
