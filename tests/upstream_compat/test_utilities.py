"""Previously unlisted, read-only POST helpers; these do not submit transactions."""
import base64
import json

import pytest

from bitcoin_wire import Reader, compact
from compat import same

pytestmark = pytest.mark.live
TX = '15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521'


@pytest.mark.parametrize('outpoints', [[], [{'txid': '0'*64, 'vout': 0}], [{'txid': '0'*64, 'vout': 0}]*2])
def test_prevouts_preserves_positions_and_nulls(target, outpoints):
    code, _, body = target.request('/api/v1/prevouts', 'POST', json.dumps(outpoints).encode(), 'application/json')
    assert code == 200
    assert json.loads(body) == [None] * len(outpoints)


def test_local_cpfp_calculation(target, references):
    body = json.dumps([dict(txid='1'*64, weight=400, sigops=0, fee=1000, vin=[], vout=[])]).encode()
    args = ('/api/v1/cpfp', 'POST', body, 'application/json')
    actual, _, data = target.request(*args)
    expected, _, reference = references['mempool'].request(*args)
    assert actual == expected == 200
    same(json.loads(data), json.loads(reference))
    result = json.loads(data)
    assert len(result) == 1 and result[0]['effectiveFeePerVsize'] == 10


@pytest.mark.parametrize('encoding', ['hex', 'base64'])
def test_psbt_addparents_preserves_unsigned_transaction(target, references, encoding):
    # One unsigned input spending a known historical transaction. It cannot be broadcast.
    unsigned = (b'\x02\x00\x00\x00\x01' + bytes.fromhex(TX)[::-1] +
                b'\x00\x00\x00\x00\x00\xff\xff\xff\xff' +
                b'\x01' + bytes(8) + b'\x01\x6a' + bytes(4))
    psbt = b'psbt\xff\x01\x00' + compact(len(unsigned)) + unsigned + b'\x00\x00\x00'
    payload = psbt.hex().encode() if encoding == 'hex' else base64.b64encode(psbt)
    code, _, body = target.request('/api/v1/psbt/addparents', 'POST', payload)
    assert code == 200, body[:300]
    result = bytes.fromhex(body.decode()) if encoding == 'hex' else base64.b64decode(body, validate=True)
    reader = Reader(result)
    assert reader.read(5) == b'psbt\xff'

    def mapping():
        result = {}
        while True:
            key = reader.blob()
            if not key:
                return result
            assert key not in result
            result[key] = reader.blob()

    global_map, input_map, output_map = mapping(), mapping(), mapping()
    reader.end()
    assert global_map[b'\x00'] == unsigned
    assert input_map[b'\x00'].hex() == references['esplora'].text('/api/tx/' + TX + '/hex', cache=True)


def test_testmempoolaccept_does_not_broadcast(target, references):
    raw = references['esplora'].text('/api/tx/' + TX + '/hex', cache=True)
    code, _, body = target.request('/api/v1/txs/test', 'POST', json.dumps([raw]).encode(), 'application/json')
    assert code == 200
    result = json.loads(body)
    assert len(result) == 1 and result[0]['txid'] == TX and result[0]['allowed'] is False
    assert TX not in target.json('/api/mempool/txids')
