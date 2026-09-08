"""Mempool REST invariants from the pinned backend handlers and API documentation."""
import math
import re
from urllib.parse import urlsplit, parse_qs

from compat import hash_id, number


def numeric_fields(value, keys):
    assert isinstance(value, dict)
    for key in keys.split():
        number(value[key])


def rows(value, keys, nonempty=False):
    assert isinstance(value, list)
    if nonempty:
        assert value, 'Historical series unexpectedly empty'
    for row in value:
        numeric_fields(row, keys)


def cpfp(value):
    assert isinstance(value, dict)
    assert isinstance(value['ancestors'], list)
    for key in ('ancestors', 'descendants'):
        if key in value:
            assert isinstance(value[key], list)
            assert len({tx['txid'] for tx in value[key]}) == len(value[key])
            for tx in value[key]:
                hash_id(tx['txid'])
                numeric_fields(tx, 'weight fee')
    if value.get('bestDescendant') is not None:
        hash_id(value['bestDescendant']['txid'])
        numeric_fields(value['bestDescendant'], 'weight fee')
    for key in ('effectiveFeePerVsize', 'sigops', 'adjustedVsize', 'fee'):
        if key in value:
            number(value[key])


def replacement(value, ancestors=()):
    assert isinstance(value, dict)
    tx = value['tx']
    hash_id(tx['txid'])
    assert tx['txid'] not in ancestors, 'Cyclic replacement tree'
    numeric_fields(tx, 'fee vsize value rate')
    assert tx['vsize'] > 0
    assert type(tx['rbf']) is bool
    number(value['time'], True)
    assert type(value['fullRbf']) is bool
    assert isinstance(value['replaces'], list)
    for child in value['replaces']:
        replacement(child, ancestors + (tx['txid'],))
        assert child['time'] <= value['time']
        if 'interval' in child:
            assert child['interval'] == value['time'] - child['time']


def mempool_contract(path, value):
    url = urlsplit(path)
    path, query = url.path, parse_qs(url.query)
    if path == '/api/v1/difficulty-adjustment':
        numeric_fields(value, 'progressPercent estimatedRetargetDate remainingBlocks remainingTime nextRetargetHeight timeAvg adjustedTimeAvg')
        assert 0 <= value['progressPercent'] <= 100
        assert 0 <= value['remainingBlocks'] <= 2016
        assert value['nextRetargetHeight'] % 2016 == 0
        for key in ('difficultyChange', 'previousRetarget', 'timeOffset'):
            assert type(value[key]) in (int, float) and math.isfinite(value[key])
    elif path == '/api/v1/prices':
        numeric_fields(value, 'time USD EUR GBP CAD CHF AUD JPY')
    elif path == '/api/v1/historical-price':
        assert isinstance(value, dict) and isinstance(value['prices'], list)
        if query.get('timestamp') == ['1500000000']:
            assert value['prices'], 'Known historical price missing'
        currency = query.get('currency', ['USD'])[0]
        rows(value['prices'], 'time ' + currency)
        numeric_fields(value['exchangeRates'], 'USDEUR USDGBP USDCAD USDCHF USDAUD USDJPY')
    elif path.startswith('/api/v1/cpfp/'):
        cpfp(value)
    elif path in ('/api/v1/replacements', '/api/v1/fullrbf/replacements'):
        assert isinstance(value, list)
        for tree in value:
            replacement(tree)
            if '/fullrbf/' in path:
                assert tree['fullRbf'] is True
    elif re.fullmatch('/api/v1/tx/[0-9a-f]{64}/rbf', path):
        assert isinstance(value, dict)
        if value.get('replacements') is not None:
            replacement(value['replacements'])
        assert value['replaces'] is None or isinstance(value['replaces'], list)
        for txid in value['replaces'] or []:
            hash_id(txid)
    elif path == '/api/v1/transaction-times':
        assert isinstance(value, list)
        assert len(value) == len(query.get('txId[]', []))
        for timestamp in value:
            number(timestamp, True)
    elif re.fullmatch('/api/v1/mining/pools/[^/]+', path):
        numeric_fields(value, 'blockCount lastEstimatedHashrate')
        assert isinstance(value['pools'], list) and value['pools']
        for pool in value['pools']:
            numeric_fields(pool, 'poolId blockCount rank emptyBlocks')
            assert isinstance(pool['name'], str) and isinstance(pool['slug'], str)
            assert pool['emptyBlocks'] <= pool['blockCount']
        assert sum(pool['blockCount'] for pool in value['pools']) == value['blockCount']
    elif re.fullmatch('/api/v1/mining/pool/[^/]+', path):
        pool = value['pool']
        numeric_fields(pool, 'id')
        for key in ('name', 'slug', 'link'):
            assert isinstance(pool[key], str)
        for key in ('addresses', 'regexes'):
            assert isinstance(pool[key], list) and all(isinstance(s, str) for s in pool[key])
        for key in ('all', '24h', '1w'):
            number(value['blockCount'][key], True)
            number(value['blockShare'][key])
            assert value['blockShare'][key] <= 1
        number(value['estimatedHashrate'])
        if value.get('reportedHashrate') is not None:
            number(value['reportedHashrate'])
    elif re.fullmatch('/api/v1/mining/(hashrate/pools/[^/]+|pool/[^/]+/hashrate)', path):
        rows(value, 'timestamp avgHashrate share')
        for row in value:
            assert row['share'] <= 1 and isinstance(row['poolName'], str)
    elif re.fullmatch('/api/v1/mining/hashrate/[^/]+', path):
        numeric_fields(value, 'currentHashrate currentDifficulty')
        rows(value['hashrates'], 'timestamp avgHashrate', nonempty=True)
        rows(value['difficulty'], 'timestamp difficulty height')
    elif path.startswith('/api/v1/mining/difficulty-adjustments'):
        assert isinstance(value, list)
        for row in value:
            assert isinstance(row, list) and len(row) == 4
            for item in row:
                number(item)
            assert row[1] % 2016 == 0
        assert all(a[1] > b[1] for a, b in zip(value, value[1:]))
    elif path.startswith('/api/v1/mining/reward-stats/'):
        numeric_fields(value, 'startBlock endBlock')
        assert value['endBlock'] - value['startBlock'] + 1 == int(path.rsplit('/', 1)[1])
        for key in ('totalReward', 'totalFee', 'totalTx'):
            assert isinstance(value[key], str) and value[key].isdigit()
        assert int(value['totalReward']) >= int(value['totalFee'])
    elif re.fullmatch('/api/v1/mining/blocks/(fees|rewards|fee-rates)/[^/]+', path):
        kind = path.split('/')[-2]
        fields = {'fees': 'avgFees', 'rewards': 'avgRewards',
                  'fee-rates': 'avgFee_0 avgFee_10 avgFee_25 avgFee_50 avgFee_75 avgFee_90 avgFee_100'}[kind]
        rows(value, 'timestamp avgHeight ' + fields, nonempty=True)
        if kind == 'fee-rates':
            for row in value:
                rates = [row[key] for key in fields.split()]
                assert rates == sorted(rates)
    elif path.startswith('/api/v1/mining/blocks/sizes-weights/'):
        rows(value['sizes'], 'timestamp avgHeight avgSize', nonempty=True)
        rows(value['weights'], 'timestamp avgHeight avgWeight', nonempty=True)
        assert len(value['sizes']) == len(value['weights'])
        for size, weight in zip(value['sizes'], value['weights']):
            assert size['avgHeight'] == weight['avgHeight'] and size['timestamp'] == weight['timestamp']
            # SQL averages may be independently rounded to whole bytes.
            assert size['avgSize'] - 1 <= weight['avgWeight'] <= 4 * size['avgSize'] + 4
