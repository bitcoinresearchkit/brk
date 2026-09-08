"""Small independent Bitcoin wire decoder used only by compatibility validators.

Formats: Bitcoin Core serialize.h, primitives/transaction.h and merkleblock.cpp.
No Bitview serializers, SDK models, or endpoint data are used for decoding.
"""
from compat import digest, same


class Reader:
    def __init__(self, data):
        self.data, self.pos = data, 0

    def read(self, size):
        assert 0 <= size <= len(self.data) - self.pos, 'Truncated Bitcoin payload'
        result = self.data[self.pos:self.pos + size]
        self.pos += size
        return result

    def uint(self, size):
        return int.from_bytes(self.read(size), 'little')

    def compact(self):
        first = self.uint(1)
        if first < 253:
            return first
        size = {253: 2, 254: 4, 255: 8}[first]
        value = self.uint(size)
        assert value >= {2: 253, 4: 65536, 8: 4294967296}[size], 'Noncanonical CompactSize'
        return value

    def blob(self):
        return self.read(self.compact())

    def end(self):
        assert self.pos == len(self.data), 'Trailing Bitcoin payload bytes'


def compact(value):
    if value < 253:
        return bytes([value])
    size, tag = (2, 253) if value <= 65535 else (4, 254) if value <= 4294967295 else (8, 255)
    return bytes([tag]) + value.to_bytes(size, 'little')


def read_transaction(reader):
    start = reader.pos
    version = reader.read(4)
    count = reader.compact()
    witness = count == 0
    if witness:
        assert reader.uint(1) == 1, 'Unknown witness flags'
        count = reader.compact()
    assert 0 < count <= len(reader.data) // 41
    stripped = version + compact(count)
    inputs = []
    for _ in range(count):
        begin = reader.pos
        txid = reader.read(32)[::-1].hex()
        vout = reader.uint(4)
        script = reader.blob()
        sequence = reader.uint(4)
        inputs.append(dict(txid=txid, vout=vout, scriptsig=script.hex(), sequence=sequence))
        stripped += reader.data[begin:reader.pos]
    count = reader.compact()
    assert 0 < count <= len(reader.data) // 9
    stripped += compact(count)
    outputs = []
    for _ in range(count):
        begin = reader.pos
        value, script = reader.uint(8), reader.blob()
        outputs.append(dict(value=value, scriptpubkey=script.hex()))
        stripped += reader.data[begin:reader.pos]
    if witness:
        for vin in inputs:
            count = reader.compact()
            assert count <= len(reader.data) - reader.pos
            vin['witness'] = [reader.blob().hex() for _ in range(count)]
    locktime = reader.read(4)
    stripped += locktime
    raw = reader.data[start:reader.pos]
    return dict(txid=digest(stripped)[::-1].hex(), wtxid=digest(raw)[::-1].hex(),
                version=int.from_bytes(version, 'little', signed=True),
                locktime=int.from_bytes(locktime, 'little'), vin=inputs, vout=outputs,
                size=len(raw), weight=3 * len(stripped) + len(raw))


def decode_transaction(raw):
    reader = Reader(raw)
    tx = read_transaction(reader)
    reader.end()
    return tx


def assert_transaction_bytes(actual, raw):
    parsed = decode_transaction(raw)
    # wtxid is independently computed but is not a required Esplora JSON field.
    same(actual, {key: value for key, value in parsed.items() if key != 'wtxid'})


def decode_block(raw):
    reader = Reader(raw)
    header = reader.read(80)
    count = reader.compact()
    assert 0 < count <= len(raw) // 60
    txs = [read_transaction(reader) for _ in range(count)]
    reader.end()
    return header, txs


def verify_merkleblock(raw, txid, block_hash):
    """Consume BIP37 partial tree and verify root, match, counts and padding."""
    reader = Reader(raw)
    header = reader.read(80)
    assert digest(header)[::-1].hex() == block_hash
    total = reader.uint(4)
    assert 0 < total <= 4000000 // 60
    count = reader.compact()
    assert 0 < count <= total
    hashes = [reader.read(32) for _ in range(count)]
    flags = reader.blob()
    reader.end()
    bits = [bool(byte & (1 << bit)) for byte in flags for bit in range(8)]
    bit_index = hash_index = 0
    matches = []

    def width(height):
        return (total + (1 << height) - 1) >> height

    def walk(height, position):
        nonlocal bit_index, hash_index
        assert bit_index < len(bits), 'Truncated partial-tree flags'
        parent_match = bits[bit_index]
        bit_index += 1
        if height == 0 or not parent_match:
            assert hash_index < len(hashes), 'Truncated partial-tree hashes'
            node = hashes[hash_index]
            hash_index += 1
            if height == 0 and parent_match:
                matches.append((node[::-1].hex(), position))
            return node
        left = walk(height - 1, position * 2)
        if position * 2 + 1 < width(height - 1):
            right = walk(height - 1, position * 2 + 1)
            assert right != left, 'Mutated partial Merkle tree'
        else:
            right = left
        return digest(left + right)

    height = 0
    while width(height) > 1:
        height += 1
    assert walk(height, 0) == header[36:68], 'Incorrect Merkle root'
    assert hash_index == len(hashes) and (bit_index + 7) // 8 == len(flags)
    assert not any(bits[bit_index:]), 'Nonzero flag padding'
    assert len(matches) == 1 and matches[0][0] == txid, 'Proof does not match requested transaction'
    return matches[0][1]


def verify_witness_commitment(txs):
    """BIP141 commits witness bytes independently of transaction IDs/header Merkle root."""
    if not any(tx['size'] * 4 != tx['weight'] for tx in txs):
        return
    commitments = [bytes.fromhex(v['scriptpubkey'])[6:38] for v in txs[0]['vout']
                   if v['scriptpubkey'].startswith('6a24aa21a9ed') and len(v['scriptpubkey']) >= 76]
    assert commitments, 'Missing witness commitment'
    witness = txs[0]['vin'][0]['witness']
    assert len(witness) == 1 and len(bytes.fromhex(witness[0])) == 32
    nodes = [bytes(32)] + [bytes.fromhex(tx['wtxid'])[::-1] for tx in txs[1:]]
    while len(nodes) > 1:
        if len(nodes) % 2:
            nodes.append(nodes[-1])
        nodes = [digest(nodes[i]+nodes[i+1]) for i in range(0, len(nodes), 2)]
    assert digest(nodes[0] + bytes.fromhex(witness[0])) == commitments[-1], 'Incorrect witness commitment'
