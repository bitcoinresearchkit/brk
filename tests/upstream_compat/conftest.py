"""Independent HTTP clients: intentionally no Bitview SDK or server imports."""
import os

import pytest

from compat import Client


def pytest_addoption(parser):
    group = parser.getgroup('upstream compatibility')
    group.addoption('--target', default=os.getenv('BITVIEW_URL', 'http://localhost:3110'))
    group.addoption('--mempool-reference', default=os.getenv('MEMPOOL_URL', 'https://mempool.space'))
    group.addoption('--esplora-reference', default=os.getenv('ESPLORA_URL', 'https://blockstream.info'))
    group.addoption('--reference-delay', type=float, default=0.5)
    group.addoption('--http-timeout', type=float, default=30)
    group.addoption('--regtest-rpc', help='Opt in to mining/reorgs on a disposable regtest wallet RPC URL')
    group.addoption('--regtest-cookie', help='Bitcoin Core regtest RPC cookie file')
    group.addoption('--state-timeout', type=float, default=30)
    group.addoption('--regtest-fixtures', default=None, help='JSON file of funded regtest transaction/package fixtures')


@pytest.fixture(scope='session')
def target(pytestconfig):
    return Client(pytestconfig.getoption('--target'), timeout=pytestconfig.getoption('--http-timeout'))


@pytest.fixture(scope='session')
def references(pytestconfig):
    return {name: Client(pytestconfig.getoption('--'+name+'-reference'),
                         delay=pytestconfig.getoption('--reference-delay'),
                         timeout=pytestconfig.getoption('--http-timeout'))
            for name in ('mempool', 'esplora')}


HEIGHTS = [0, 100, 100000, 400000, 630000, 800000]


@pytest.fixture(params=HEIGHTS, ids=lambda h: f'height-{h}')
def chain_case(request, references):
    ref = references['mempool']
    height = request.param
    bh = ref.text(f'/api/block-height/{height}', cache=True)
    info = ref.json(f'/api/block/{bh}', cache=True)
    txids = ref.json(f'/api/block/{bh}/txids', cache=True)
    return ref, bh, info, txids

