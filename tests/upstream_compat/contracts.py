"""Documented Bitcoin response invariants independent of a live reference."""
import re
from urllib.parse import urlsplit, parse_qs

from compat import block, hash_id, number, status, transaction


def core_contract(path, value):
    """Checks that remain meaningful even when a live reference has empty arrays."""
    url = urlsplit(path)
    path = url.path
    if path.startswith(('/api/v1/tx/', '/api/v1/mempool', '/api/v1/address', '/api/v1/scripthash')):
        path = path.replace('/api/v1/', '/api/', 1)
    if path == '/api/v1/txs/outspends':
        txids = parse_qs(url.query).get('txids', [''])[0].split(',')
        assert isinstance(value, list) and len(value) == len(txids)
        for spends in value:
            core_contract('/api/tx/' + txids[0] + '/outspends', spends)
        return
    if re.fullmatch(r'/api/(v1/)?block/[0-9a-f]{64}', path):
        block(value)
    elif re.fullmatch(r'/api/(v1/)?blocks(?:/\d+)?', path):
        assert isinstance(value, list) and 0 < len(value) <= (15 if '/v1/' in path else 10)
        for item in value:
            block(item)
        assert all(a['height'] == b['height'] + 1 for a, b in zip(value, value[1:]))
    elif re.fullmatch(r'/api/tx/[0-9a-f]{64}', path):
        transaction(value)
    elif re.search(r'/txs(?:/chain(?:/[0-9a-f]{64})?|/mempool|/\d+)?$', path):
        assert isinstance(value, list)
        assert len({tx['txid'] for tx in value}) == len(value)
        for tx in value:
            transaction(tx)
        if '/chain' in path or '/block/' in path:
            assert len(value) <= 25
            assert all(tx['status']['confirmed'] for tx in value)
        elif path.endswith('/mempool'):
            assert len(value) <= 50
            assert all(not tx['status']['confirmed'] for tx in value)
        else:
            assert sum(tx['status']['confirmed'] for tx in value) <= 25
            assert sum(not tx['status']['confirmed'] for tx in value) <= 50
    elif re.fullmatch(r'/api/(address|scripthash)/[^/]+', path):
        for kind in ('chain_stats', 'mempool_stats'):
            stats = value[kind]
            for key in ('tx_count', 'funded_txo_count', 'funded_txo_sum', 'spent_txo_count', 'spent_txo_sum'):
                number(stats[key], True)
        # Mempool spends can consume confirmed outputs, so only chain totals order.
        assert value['chain_stats']['spent_txo_sum'] <= value['chain_stats']['funded_txo_sum']
    elif path.endswith('/utxo'):
        assert isinstance(value, list)
        assert len({(v['txid'], v['vout']) for v in value}) == len(value)
        for item in value:
            hash_id(item['txid'])
            number(item['vout'], True)
            number(item['value'], True)
            status(item['status'])
    elif '/outspend' in path:
        items = value if path.endswith('/outspends') else [value]
        assert isinstance(items, list)
        for item in items:
            assert type(item['spent']) is bool
            if item['spent']:
                hash_id(item['txid'])
                number(item['vin'], True)
                status(item['status'])
    elif path == '/api/mempool':
        for key in ('count', 'vsize', 'total_fee'):
            number(value[key], True)
        histogram = value['fee_histogram']
        assert isinstance(histogram, list)
        for pair in histogram:
            assert len(pair) == 2
            number(pair[0])
            number(pair[1], True)
        assert all(a[0] >= b[0] for a, b in zip(histogram, histogram[1:]))
    elif path == '/api/mempool/txids' or path.endswith('/txids') and '/block/' in path:
        assert isinstance(value, list) and len(set(value)) == len(value)
        for item in value:
            hash_id(item)
    elif path == '/api/mempool/recent':
        assert isinstance(value, list) and len(value) <= 10
        for item in value:
            hash_id(item['txid'])
            for key in ('fee', 'vsize', 'value'):
                number(item[key], True)
    elif path in ('/api/v1/fees/recommended', '/api/v1/fees/precise'):
        keys = ('fastestFee', 'halfHourFee', 'hourFee', 'economyFee', 'minimumFee')
        for key in keys:
            number(value[key])
        assert all(value[a] >= value[b] for a, b in zip(keys, keys[1:]))
    elif path == '/api/fee-estimates':
        assert isinstance(value, dict)
        for target in list(range(1, 26)) + [144, 504, 1008]:
            number(value[str(target)])
    elif path == '/api/v1/fees/mempool-blocks':
        assert isinstance(value, list)
        for item in value:
            for key in ('blockSize', 'blockVSize', 'nTx', 'totalFees', 'medianFee'):
                number(item[key])
            assert isinstance(item['feeRange'], list) and item['feeRange']
            for fee in item['feeRange']:
                number(fee)
            assert item['feeRange'] == sorted(item['feeRange'])
    elif path == '/api/block-template':
        for key in ('version', 'height', 'curtime', 'bits', 'previousblockhash', 'transactions', 'coinbasevalue'):
            assert key in value
        hash_id(value['previousblockhash'])
        for tx in value['transactions']:
            hash_id(tx['txid'])
            bytes.fromhex(tx['data'])
    elif path.startswith('/api/address-prefix/'):
        assert isinstance(value, list) and len(value) <= 10
        assert all(v.startswith(path.rsplit('/', 1)[1]) for v in value)

