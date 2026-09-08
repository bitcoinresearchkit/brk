"""Opt-in Bitcoin Core fixture driver. Never used by the default mainnet suite."""
import base64
import json
import time
from decimal import Decimal
from urllib.request import Request, urlopen


GENESIS = '0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206'


class Regtest:
    def __init__(self, rpc_url, auth, target, wait=30):
        self.rpc_url, self.auth, self.target, self.wait = rpc_url, auth, target, wait
        assert self.rpc('getblockchaininfo')['chain'] == 'regtest'
        assert self.rpc('getblockhash', 0) == GENESIS
        assert target.text('/api/block-height/0') == GENESIS, 'Target is not regtest'
        # The caller explicitly selects a disposable wallet-scoped RPC endpoint.
        self.rpc('getwalletinfo')

    def rpc(self, method, *params):
        request = Request(self.rpc_url, data=json.dumps(dict(jsonrpc='2.0', id=1, method=method, params=params)).encode(),
                          headers={'Content-Type': 'application/json',
                                   'Authorization': 'Basic ' + base64.b64encode(self.auth.encode()).decode()})
        with urlopen(request, timeout=30) as response:
            result = json.load(response)
        assert result.get('error') is None, f'Regtest RPC {method}: {result.get("error")}'
        return result['result']

    def synced(self):
        expected = self.rpc('getbestblockhash')
        self.until(lambda: self.target.text('/api/blocks/tip/hash') == expected, 'indexed regtest chain tip')

    def until(self, condition, description):
        deadline = time.monotonic() + self.wait
        while True:
            if condition():
                return
            assert time.monotonic() < deadline, f'Timed out waiting for {description}'
            time.sleep(0.2)

    def mine(self, count=1):
        hashes = self.rpc('generatetoaddress', count, self.rpc('getnewaddress'))
        self.synced()
        return hashes

    def signed(self, utxo, address, fee=1000, sequence=0xfffffffd):
        sats = int(Decimal(str(utxo['amount'])) * 100000000) - fee
        raw = self.rpc('createrawtransaction', [dict(txid=utxo['txid'], vout=utxo['vout'], sequence=sequence)],
                       {address: float(Decimal(sats) / 100000000)})
        signed = self.rpc('signrawtransactionwithwallet', raw)
        assert signed['complete']
        decoded = self.rpc('decoderawtransaction', signed['hex'])
        return dict(hex=signed['hex'], txid=decoded['txid'], sats=sats, address=address)

    def funded(self, address=None, fee=1000):
        # Mine enough maturity; only the explicitly supplied disposable regtest is mutated.
        coins = [u for u in self.rpc('listunspent', 101) if u['spendable']]
        if not coins:
            self.mine(101)
            coins = [u for u in self.rpc('listunspent', 101) if u['spendable']]
        assert coins
        coin = coins[0]
        self.rpc('lockunspent', False, [dict(txid=coin['txid'], vout=coin['vout'])])
        return coin, self.signed(coin, address or self.rpc('getnewaddress'), fee)

    def send(self, tx):
        code, _, body = self.target.request('/api/tx', 'POST', tx['hex'].encode())
        assert code == 200 and body.decode().strip() == tx['txid'], f'Broadcast HTTP {code}: {body[:200]!r}'
        self.until(lambda: tx['txid'] in self.target.json('/api/mempool/txids'), 'broadcast in target mempool')

    def output(self, tx):
        return dict(txid=tx['txid'], vout=0, amount=float(Decimal(tx['sats']) / 100000000))
