SRC = """
---
- - college
  - -380608299.3165369
  - closely: 595052867
    born: false
    stomach: true
    expression: true
    chosen: 34749965
    somebody: false
  - positive
  - true
  - false
- price
- 2018186817
- average
- young
- -1447308110

"""

import ryaml

for _ in range(1000):
    ryaml.loads(SRC)
