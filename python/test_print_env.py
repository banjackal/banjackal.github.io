import os
from unittest import mock

import print_env as sut

@mock.patch.dict(os.environ, {'foo': 'fooey'}, clear=True)
def test_print_env():
    res = sut.getEnv()
    assert 'bar' in res


