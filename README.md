## SUFFIX ARRAY

build suffix array

---
- [x] n-grams comparison sort, by using built-in sort_by(), which takes O(N(logN)^2) time

- [x] n-grams non-comparison sort, by using Counting Sort, which takes
  O(NlogN) time.

- [ ] SAIS

---

```rust
/* simulation
    s : "banana$"
    sa : [0, 1, 2, 3, 4, 5, 6]
    rank : [b, a, n, a, n, a, $]
    pos : [0, 1, 2, 3, 4, 5, 6]

    1. gap == 1
        freq[rank[i+gap]]
        freq : [$: 2, a: 3, n: 2]
        running sum : [$: 2, a: 5, n: 7]
        pos[freq[rank[i+gap]]] = i (reverse order)
        pos : [5, 6, 0, 2, 4, 1, 3]

        freq[rank[i]]
        freq : [$: 1, a: 3, b: 1, n: 2]
        running sum : [$:1, a: 4, b: 5, n: 7]
        sa[freq[rank[pos[i]]] = pos[i]
        sa : [6, 5, 1, 3, 0, 2, 4]

        nrank[sa[i+1]] = nrank[sa[i]] + comp
        nrank : [3, 2, 4, 2, 4, 1, 0]
        rank = nrank

    2. gap == 2
        freq : [0: 3, 1: 2, 3: 2]
        running sum : [0: 3, 1: 5, 3: 7]
        pos[freq[rank[i+gap]]] = i
        pos : [4, 5, 6, 1, 3, 0, 2]

        freq[rank[i]]
        freq : [0: 1, 1: 3, 2: 1, 3: 2]
        running sum : [0: 1, 1: 4, 2: 5, 3: 7]
        sa[freq[rank[pos[i]]]] = pos[i]
        sa : [6, 5, 3, 1, 0, 4, 2]
        
        nrank[sa[i+1]] = nrank[sa[i]] + comp
        nrank : [4, 3, 6, 2, 5, 1, 0]

        rank = nrank
        rank[sa[n-1]] == rank[2] == 6 == n-1 // break

*/
```
