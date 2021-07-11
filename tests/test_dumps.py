import ryaml

def test_dumps_none():
    assert ryaml.dumps(None) == '---\n~\n'

def test_dumps_key():
    assert ryaml.dumps({ 'key': None }) == '---\nkey: ~\n'

def test_dumps_key_value():
    assert ryaml.dumps({ 'key': 4 }) == '---\nkey: 4\n'

def test_dumps_key_sequence():
    assert ryaml.dumps({ 'key': [4, 5] }) == '---\nkey:\n  - 4\n  - 5\n'
