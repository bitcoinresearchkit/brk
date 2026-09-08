"""Historical mining amounts checked against independently decoded Bitcoin bytes."""
import pytest

from bitcoin_wire import decode_block
from compat import same

pytestmark = pytest.mark.live


@pytest.mark.parametrize('height', [209999, 210000, 419999, 420000, 629999, 630000, 839999, 840000])
def test_block_reward_across_halvings(target, references, height):
    ref = references['esplora']
    bh = ref.text(f'/api/block-height/{height}', cache=True)
    code, _, raw = ref.request('/api/block/' + bh + '/raw', cache=True)
    assert code == 200
    _, txs = decode_block(raw)
    actual = target.json('/api/v1/block/' + bh)
    assert actual['height'] == height
    reward = sum(output['value'] for output in txs[0]['vout'])
    assert actual['extras']['reward'] == reward
    subsidy = 5000000000 >> (height // 210000)
    # Miners may claim less than the allowed subsidy + fees, never more.
    assert reward <= subsidy + actual['extras']['totalFees']
    assert actual['extras']['coinbaseRaw'] == txs[0]['vin'][0]['scriptsig']


@pytest.mark.parametrize('count', [1, 2, 10])
def test_reward_stats_sum_actual_coinbases(target, references, count):
    result = target.json('/api/v1/mining/reward-stats/' + str(count))
    assert result['endBlock'] - result['startBlock'] + 1 == count
    reward = total_tx = 0
    for height in range(result['startBlock'], result['endBlock']+1):
        ref = references['esplora']
        bh = ref.text('/api/block-height/' + str(height), cache=True)
        info = ref.json('/api/block/' + bh, cache=True)
        coinbase = ref.json('/api/block/' + bh + '/txs/0', cache=True)[0]
        reward += sum(v['value'] for v in coinbase['vout'])
        total_tx += info['tx_count']
    assert int(result['totalReward']) == reward
    assert int(result['totalTx']) == total_tx


def test_timestamp_lookup_hits_exact_block(target, references):
    ref = references['esplora']
    bh = ref.text('/api/block-height/800000', cache=True)
    info = ref.json('/api/block/' + bh, cache=True)
    result = target.json('/api/v1/mining/blocks/timestamp/' + str(info['timestamp']))
    # Pin a known monotonic boundary rather than assuming all Bitcoin timestamps sort.
    expected = references['mempool'].json('/api/v1/mining/blocks/timestamp/' + str(info['timestamp']), cache=True)
    same(result, expected)
