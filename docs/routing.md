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

**Result**: The cumulative false-positive rate remains manageable.

---

## 4. IPv6 Prefix Encoding

### 4.1 Address Structure

| Global Routing Prefix | Area | Domain | Sub-Domain | Expert |
|-------|------|------|------|------|
| (48 Bit) | (16 B) | (16 Bit) | (16 Bit) | (32 Bit) |


### 4.2 Hashing

Each level hashes the prefix of the next level:

Level 1: hash(area_prefix)
Level 2: hash(domain_prefix)
Level 3: hash(subdomain_prefix)
Level 4: hash(expert_id)
Level 5: hash(metadata)


### 4.3 Expand Function

The `expand(c)` function returns the child prefixes of c:

```
expand(2001:db8:physics::/48) → {
2001:db8:physics:acoustics::/64,
2001:db8:physics:optics::/64,
...
}
```


---

## 5. Multicast Discovery

### 5.1 Multicast Groups

Each area/domain/subdomain has a multicast group:

```
ff0e:2001:db8:physics:acoustics::1
```

### 5.2 Registration

Experts register to their group via MLD (Multicast Listener Discovery):

```
Multicast: ff0e:2001:db8:physics:acoustics::1
Payload: {query: "...", nonce: "..."}
```

### 5.4 Response

Experts respond via unicast:

```
Unicast: 2001:db8:expert::1
Payload: {response: "...", nonce: "...", signature: "..."}
```

---

## 6. Anycast Load Balancing

### 6.1 Anycast Address

Popular experts share an anycast address:

```
2001:db8:physics:acoustics:popular::1
```

### 6.2 Routing

BGP routes to the nearest instance:

```
AS1 → AS2 → AS3 (nearest)
```

### 6.3 Failover

If an instance fails, BGP withdraws the route:

```
Withdraw: 2001:db8:physics:acoustics:popular::1
```

Traffic is rerouted to the next-nearest instance.

---

## 7. Caching

### 7.1 Semantic Cache

The Router LLM maintains a semantic cache:

```
Key: embedding(query)
Value: {prefixes: [...], timestamp: ...}
```

### 7.2 Exact Cache

The Aggregator LLM maintains an exact cache:

```
Key: hash(query, expert_set)
Value: {response: "...", timestamp: ...}
```

### 7.3 Cache Invalidation

Caches are invalidated:

- **Time-based**: After T seconds
- **Event-based**: When experts change
- **Manual**: Via admin API

---

## 8. Performance

### 8.1 Latency Breakdown

| Component | Latency |
|-----------|---------|
| Router LLM | 100–500 ms |
| Bloom filter (5 levels) | ~500 ns |
| Multicast roundtrip | 10–100 ms |
| Expert inference | 100–1000 ms |
| Aggregator LLM | 200–1000 ms |
| **Total** | **~500–2500 ms** |

### 8.2 Throughput

| Metric | Value |
|--------|-------|
| Queries/second (single router) | ~100 |
| Queries/second (federated) | ~10,000 |
| Experts addressable | 10¹²+ |
| Bloom-filter memory | 625 MB |

---

## 9. Security

### 9.1 Sybil Resistance

- **Proof-of-stake**: Experts stake tokens
- **Proof-of-reputation**: Experts build reputation over time
- **Proof-of-work**: Experts solve puzzles (expensive)

### 9.2 Malicious Experts

- **Reputation**: Low-reputation experts are deprioritized
- **Sandboxing**: Docker containers with restricted capabilities
- **Verification**: Multi-LLM consensus for critical queries

### 9.3 Eclipse Attacks

- **Diverse peers**: Experts connect to diverse peers
- **Random routing**: Some queries routed randomly
- **Monitoring**: Anomaly detection

---

## 10. Future Work

- **Federated routing**: Multiple routers
- **Adaptive Bloom filters**: Dynamic resizing
- **Learned Bloom filters**: Combining Bloom with ML
- **Cross-area routing**: Routing between areas
- **Standardization**: IETF proposal

---

## References

See `README.md` for the full reference list.
