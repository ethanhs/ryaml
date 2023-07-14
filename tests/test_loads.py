import pytest
import ryaml

def test_loads_empty():
    ryaml.loads('') is None

def test_loads_key():
    assert ryaml.loads('''
    key:

    ''') == { 'key': None }

def test_loads_key_value():
    assert ryaml.loads('''
    key:
        4

    ''') == { 'key': 4 }

def test_loads_key_sequence():
    assert ryaml.loads('''
    key:
        - 4
        - 5

    ''') == { 'key': [4, 5] }


def test_duplicate_key():
    with pytest.raises(ryaml.InvalidYamlError):
        ryaml.loads("""
        a:
            - b
            - c

        a:
            q: a
            q: b
        """)
