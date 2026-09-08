"""Strict wire comparisons with only additive object fields permitted."""
import hashlib
import json
import math
import time
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

INVENTORY = json.loads(Path(__file__).with_name('endpoints.json').read_text())
ENDPOINTS = INVENTORY['endpoints']


class Client:
    def __init__(self, base, delay=0, timeout=30):
        self.base = base.rstrip('/')
        self.delay = delay
        self.timeout = timeout
        self.last = 0
        self.cache = {}

    def request(self, path, method='GET', body=None, content_type='text/plain', cache=False, headers=None):
        key = (method, path, body, tuple(sorted((headers or {}).items())))
        if cache and key in self.cache:
            return self.cache[key]
        time.sleep(max(0, self.delay - (time.monotonic() - self.last)))
        self.last = time.monotonic()
        req = Request(self.base + path, data=body, method=method,
                      headers={'User-Agent': 'bitview-upstream-compat/1.0',
                               'Content-Type': content_type, 'Accept-Encoding': 'identity', **(headers or {})})
        try:
            response = urlopen(req, timeout=self.timeout)
        except HTTPError as error:
            response = error
        with response:
            result = (response.status, response.headers, response.read())
        if cache and result[0] == 200:
            self.cache[key] = result
        return result

    def json(self, path, cache=False):
        status, headers, body = self.request(path, cache=cache)
        assert status == 200, f'{self.base}{path}: HTTP {status}: {body[:300]!r}'
        assert 'json' in headers.get('Content-Type', ''), f'{path}: expected JSON Content-Type'
        return decode_json(body)

    def text(self, path, cache=False):
        status, _, body = self.request(path, cache=cache)
        assert status == 200, f'{self.base}{path}: HTTP {status}: {body[:300]!r}'
        return body.decode().strip()


def same(actual, expected, path='$'):
    """Exact ordered values; permit extra keys at any object depth, never missing ones."""
    if isinstance(expected, dict):
        assert isinstance(actual, dict), f'{path}: expected object, got {actual!r}'
        for key, value in expected.items():
            assert key in actual, f'{path}: missing {key}'
            same(actual[key], value, f'{path}.{key}')
    elif isinstance(expected, list):
        assert isinstance(actual, list), f'{path}: expected array'
        assert len(actual) == len(expected), f'{path}: length {len(actual)} != {len(expected)}'
        for i, (a, e) in enumerate(zip(actual, expected)):
            same(a, e, f'{path}[{i}]')
    elif type(expected) in (int, float):
        assert type(actual) in (int, float), f'{path}: expected number, got {actual!r}'
        assert math.isfinite(actual), f'{path}: nonfinite number'
        # Integer monetary amounts remain exact; allow only floating arithmetic noise.
        equal = actual == expected if type(expected) is int else math.isclose(actual, expected, rel_tol=1e-12, abs_tol=1e-12)
        assert equal, f'{path}: {actual!r} != {expected!r}'
    else:
        assert type(actual) is type(expected) and actual == expected, f'{path}: {actual!r} != {expected!r}'


def shape(actual, expected, path='$'):
    """Validate every item against observed variants, without comparing live values/counts.

    Empty reference arrays and nulls supply no item/type evidence. Dedicated core
    contracts below still validate those responses. No coercion of strings/bools.
    """
    if expected is None:
        return
    if actual is None and path.rsplit('.', 1)[-1] in {'bestDescendant', 'replacements', 'replaces', 'reportedHashrate'}:
        return
    if isinstance(expected, dict):
        assert isinstance(actual, dict), f'{path}: expected object'
        conditional = set()
        if type(expected.get('confirmed')) is bool:
            status(actual)
            if not actual['confirmed']:
                conditional = {'block_height', 'block_hash', 'block_time'}
        if type(expected.get('spent')) is bool:
            assert type(actual.get('spent')) is bool, f'{path}: expected spent boolean'
            if not actual['spent']:
                conditional = {'txid', 'vin', 'status'}
            else:
                hash_id(actual['txid'])
                number(actual['vin'], True)
                status(actual['status'])
        for key, value in expected.items():
            if key in conditional:
                continue
            assert key in actual, f'{path}: missing {key}'
            shape(actual[key], value, f'{path}.{key}')
    elif isinstance(expected, list):
        assert isinstance(actual, list), f'{path}: expected array'
        variants = {}
        for item in expected:
            signature = tuple(sorted(item)) if isinstance(item, dict) else type(item).__name__
            variants.setdefault(signature, item)
        for i, item in enumerate(actual):
            if not variants:
                break
            failures = []
            for variant in variants.values():
                try:
                    shape(item, variant, f'{path}[{i}]')
                    break
                except AssertionError as error:
                    failures.append(str(error))
            else:
                raise AssertionError('; '.join(failures))
    elif type(expected) in (int, float):
        assert type(actual) in (int, float) and math.isfinite(actual), f'{path}: expected finite number'
    else:
        assert type(actual) is type(expected), f'{path}: expected {type(expected).__name__}, got {actual!r}'


def number(value, integer=False):
    assert type(value) in ((int,) if integer else (int, float)), f'not a number: {value!r}'
    assert math.isfinite(value) and value >= 0, f'negative/nonfinite: {value!r}'


def hash_id(value):
    assert isinstance(value, str) and len(value) == 64
    assert all(c in '0123456789abcdef' for c in value), value


def status(value):
    assert type(value['confirmed']) is bool
    if value['confirmed']:
        hash_id(value['block_hash'])
        number(value['block_height'], True)
        number(value['block_time'], True)



def output(value):
    number(value['value'], True)
    assert value['value'] <= 2100000000000000
    for key in ('scriptpubkey', 'scriptpubkey_asm', 'scriptpubkey_type'):
        assert isinstance(value[key], str)
    bytes.fromhex(value['scriptpubkey'])
    if value['scriptpubkey_type'] in ('p2pkh', 'p2sh', 'v0_p2wpkh', 'v0_p2wsh', 'v1_p2tr'):
        assert isinstance(value['scriptpubkey_address'], str) and value['scriptpubkey_address']

def transaction(tx):
    hash_id(tx['txid'])
    assert type(tx['version']) is int
    number(tx['locktime'], True)
    assert -(1 << 31) <= tx['version'] < (1 << 31)
    assert tx['locktime'] <= 0xffffffff
    for key in ('size', 'weight', 'fee'):
        number(tx[key], True)
    assert 0 < tx['size'] <= tx['weight'] <= 4 * tx['size']
    assert isinstance(tx['vin'], list) and tx['vin']
    assert isinstance(tx['vout'], list) and tx['vout']
    for value in tx['vout']:
        output(value)
    for vin in tx['vin']:
        hash_id(vin['txid'])
        number(vin['vout'], True)
        number(vin['sequence'], True)
        assert vin['vout'] <= 0xffffffff and vin['sequence'] <= 0xffffffff
        assert type(vin['is_coinbase']) is bool
        assert isinstance(vin['scriptsig_asm'], str)
        bytes.fromhex(vin['scriptsig'])
        for witness in vin.get('witness', []):
            bytes.fromhex(witness)
        if vin['is_coinbase']:
            assert len(tx['vin']) == 1 and vin['prevout'] is None
            assert vin['vout'] == 0xffffffff and vin['txid'] == '0' * 64
            assert tx['fee'] == 0
        else:
            assert isinstance(vin['prevout'], dict)
            output(vin['prevout'])
    if not tx['vin'][0]['is_coinbase']:
        assert sum(v['prevout']['value'] for v in tx['vin']) - sum(v['value'] for v in tx['vout']) == tx['fee']
    status(tx['status'])


def block(value):
    for key in ('id', 'merkle_root'):
        hash_id(value[key])
    for key in ('height', 'timestamp', 'bits', 'nonce', 'tx_count', 'size', 'weight', 'mediantime'):
        number(value[key], True)
    assert type(value['version']) is int
    if value['height']:
        hash_id(value['previousblockhash'])
    assert value['tx_count'] > 0 and 0 < value['size'] <= value['weight'] <= 4 * value['size']


def digest(raw):
    return hashlib.sha256(hashlib.sha256(raw).digest()).digest()


def decode_json(body):
    def invalid_constant(value):
        raise ValueError('Invalid JSON numeric constant: ' + value)
    return json.loads(body, parse_constant=invalid_constant)
