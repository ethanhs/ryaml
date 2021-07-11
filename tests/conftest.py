import pytest

@pytest.fixture
def yaml_file(tmp_path):
    with open(tmp_path / 'testfile.yaml', 'wb+') as y:
        yield y
