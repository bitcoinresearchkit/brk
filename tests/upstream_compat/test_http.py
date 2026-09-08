"""Representation correctness, HEAD, and conditional-response validation ordering."""
import gzip

import pytest


pytestmark = pytest.mark.live
TX = '15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521'
# Fixed byte representations: JSON may legally contain changing extension fields.
PATHS = ['/api/block-height/100', f'/api/tx/{TX}/hex', f'/api/tx/{TX}/raw']


@pytest.mark.parametrize('path', PATHS)
def test_head_matches_get(target, path):
    code, headers, body = target.request(path)
    assert code == 200
    head_code, head_headers, head_body = target.request(path, 'HEAD')
    assert head_code == 200 and head_body == b''
    assert head_headers.get_content_type() == headers.get_content_type()
    if 'Content-Length' in head_headers:
        assert int(head_headers['Content-Length']) == len(body)


@pytest.mark.parametrize('path', PATHS)
def test_encoded_body_and_etag_revalidation(target, path):
    code, headers, body = target.request(path)
    assert code == 200
    zipped_code, zipped_headers, zipped_body = target.request(path, headers={'Accept-Encoding': 'gzip'})
    assert zipped_code == 200
    if zipped_headers.get('Content-Encoding') == 'gzip':
        assert gzip.decompress(zipped_body) == body
        assert 'accept-encoding' in zipped_headers.get('Vary', '').lower()
        if headers.get('ETag') and zipped_headers.get('ETag'):
            if not headers['ETag'].startswith('W/') and not zipped_headers['ETag'].startswith('W/'):
                assert headers['ETag'] != zipped_headers['ETag'], 'Different bytes share a strong ETag'
    else:
        assert not zipped_headers.get('Content-Encoding')
        assert zipped_body == body
    etag = headers.get('ETag')
    if etag:
        for candidate in (etag, '"unrelated", ' + etag, '*'):
            conditional, _, conditional_body = target.request(path, headers={'If-None-Match': candidate})
            assert conditional == 304 and conditional_body == b''
        fresh, _, fresh_body = target.request(path, headers={'If-None-Match': '"compat-never-matches"'})
        assert fresh == 200 and fresh_body == body


@pytest.mark.parametrize('path', ['/api/tx/not-hex', '/api/block-height/-1', '/api/tx/' + '0'*64, '/api/address/not-valid'])
def test_conditionals_cannot_bypass_validation(target, path):
    normal, _, _ = target.request(path)
    assert normal in (400, 404, 422)
    conditional, _, _ = target.request(path, headers={'If-None-Match': '*'})
    assert conditional == normal
