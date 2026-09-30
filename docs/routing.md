# Bloom-Filter Routing in Detail

**Version**: 0.1.0
**Status**: Draft
**Last Updated**: [Date]

---

## 1. Introduction

Bloom filters are probabilistic data structures for set membership testing. They answer the question "Is element X in set S?" with:

- **"Definitely not"** (no false negatives)
- **"Possibly yes"** (may include false positives)

The false-positive rate is controlled by:

- **m**: Bit array size
- **n**: Number of elements
- **k**: Number of hash functions

---

## 2. Bloom-Filter Fundamentals

### 2.1 Insertion

```
insert(x):
for i = 1 to k:
B[h_i(x)] = 1
```

### 2.2 Query

```
contains(x):
for i = 1 to k:
if B[h_i(x)] == 0:
return "definitely not"
return "possibly yes"
```

### 2.3 False-Positive Rate

The false-positive rate is:

```
f = (1 - e^(-kn/m))^k
```

Optimal k:

```
k_opt = (m/n) ln 2
```

Optimal false-positive rate:

```
f_opt = (0.6185)^(m/n)
```

### 2.4 Example

For n = 10⁷ elements and m/n = 10 bits/element:

- k_opt ≈ 7
- f_opt ≈ 0.008 (0.8%)
- Memory: 12.5 MB

---

## 3. Hierarchical Bloom-Filter Routing

### 3.1 Motivation

A single Bloom filter with 10¹² elements would require:

- 1.25 TB memory (at 10 bits/element)
- Same false-positive rate (0.8%)

This is impractical. Instead, we partition into **levels** of 10⁷ elements each.

### 3.2 Hierarchy

Level 1: Area prefixes (10⁷ entries, 12.5 MB, 0.8% FP)
Level 2: Domain prefixes (10⁷ entries, 12.5 MB, 0.8% FP)
Level 3: Sub-domain prefixes(10⁷ entries, 12.5 MB, 0.8% FP)
Level 4: Expert IDs (10⁷ entries, 12.5 MB, 0.8% FP)
Level 5: Metadata (10⁷ entries, 12.5 MB, 0.8% FP)

### 3.3 Routing Algorithm

Input: query q, hierarchy H = {B_1, ..., B_L}
Output: candidate set C

```
1: C ← {all area prefixes}
2: for l = 1 to L do
3: C' ← ∅
4: for each c ∈ C do
5: if B_l.contains(hash(c)) then
6: C' ← C' ∪ expand(c)
7: C ← C'
8: return C
```

### 3.4 Cumulative False-Positive Rate

Let C_l be the candidate set after level l, and T_l the true set.

```
|C_l| ≤ |T_l| + f_l · |C_{l-1}|
```

For L = 5, f_l = 0.01, |C_0| = 10⁷, |T_5| = 10:

```
|C_5| ≈ 10 + 0.01 · 10⁷ · (0.01)^4
≈ 10 + 10^-3
≈ 10
```
