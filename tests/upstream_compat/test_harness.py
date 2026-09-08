"""Offline regression tests for the validators, not the implementation under test."""
import pytest

from compat import ENDPOINTS, same, shape


def test_extra_fields_are_allowed_recursively():
    same({'a': [{'b': 1, 'extra': False}], 'extra': None}, {'a': [{'b': 1}]})
    shape({'a': [{'b': 1, 'extra': False}], 'extra': None}, {'a': [{'b': 2}]})


@pytest.mark.parametrize('actual,expected', [
    ({}, {'required': 1}),
    ({'a': None}, {'a': 2.5}),
    ({'a': '2'}, {'a': 2}),
    ({'a': True}, {'a': 1}),
    ({'a': 65535}, {'a': 4294967295}),
    ({'a': ['aa']}, {'a': ['aa', '']}),
    ({'a': 'computed'}, {'a': None}),
    ({'a': [2, 1]}, {'a': [1, 2]}),
    ({'fee': 100000001}, {'fee': 100000000}),
])
def test_exact_comparator_catches_real_differences(actual, expected):
    with pytest.raises(AssertionError):
        same(actual, expected)


@pytest.mark.parametrize('actual', [[{'a': 1}, {}], [{'a': True}], [{'a': None}], [{'a': '1'}]])
def test_shape_checks_every_array_item(actual):
    with pytest.raises(AssertionError):
        shape(actual, [{'a': 1}])


def test_shape_allows_live_counts_and_variant_objects():
    shape([{'confirmed': False}, {'confirmed': True, 'block_height': 800000, 'block_hash': 'a'*64, 'block_time': 1}],
          [{'confirmed': True, 'block_height': 700000, 'block_hash': 'b'*64, 'block_time': 2}, {'confirmed': False}])
    shape([], [{'a': 1}])


def test_inventory_is_complete_and_resolvable():
    ids = [e['id'] for e in ENDPOINTS]
    assert len(ids) == len(set(ids))
    for entry in ENDPOINTS:
        assert entry['method'] in ('GET', 'POST')
        if entry.get('exclude'):
            assert entry['exclude']
            continue
        assert entry['path'].startswith('/api/')
        assert '%{' not in entry['path'], entry
        assert entry['mode'] in ('shape', 'exact', 'address-page', 'broadcast', 'package')
    paths = {e['path'] for e in ENDPOINTS if not e.get('exclude')}
    assert '/api/fee-estimates' in paths
    assert '/api/txs/package' in paths
    assert len([p for p in paths if p.startswith('/api/scripthash/')]) == 6


def test_live_confirmation_and_spending_transitions_are_not_schema_errors():
    shape({'confirmed': False}, {'confirmed': True, 'block_height': 1, 'block_hash': 'a'*64, 'block_time': 1})
    shape({'spent': False}, {'spent': True, 'txid': 'a'*64, 'vin': 0, 'status': {'confirmed': False}})
    with pytest.raises(KeyError):
        shape({'confirmed': True}, {'confirmed': False})
    with pytest.raises(KeyError):
        shape({'spent': True}, {'spent': False})
