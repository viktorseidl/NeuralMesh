# System Architecture

**Version**: 0.1.0
**Status**: Draft
**Last Updated**: 30.09.2026

---

## 1. Overview

NeuralMesh is organized into five logical layers:
```
┌─────────────────────────────────────────────────┐
│ Layer 0: User Interface │
├─────────────────────────────────────────────────┤
│ Layer 1: Router LLM (semantic analysis) │
├─────────────────────────────────────────────────┤
│ Layer 2: IPv6 Bloom-Filter Routing │
├─────────────────────────────────────────────────┤
│ Layer 3: 2B Experts (distributed nodes) │
├─────────────────────────────────────────────────┤
│ Layer 4: Aggregator LLM (synthesis) │
└─────────────────────────────────────────────────┘
```

---

## 2. Layer 0: User Interface

The user submits a query via:

- **Web UI** (browser-based chat)
- **API** (REST / gRPC)
- **CLI** (command-line tool)
- **SDK** (Python, JavaScript, Rust)

The query is forwarded to the Router LLM.

---

## 3. Layer 1: Router LLM

### 3.1 Purpose

The Router LLM performs:

1. **Semantic analysis**: What is the user asking?
2. **Domain decomposition**: Which domains are relevant?
3. **Prefix selection**: Which IPv6 area prefixes should be queried?

### 3.2 Architecture

- **Model**: Initially a centralized 70B LLM (e.g., Mixtral 8x22B)
- **Interface**: gRPC
- **Caching**: Semantic cache for repeated queries
- **Federation**: Later, multiple Router LLMs via anycast

### 3.3 Example

Input:

```
"Analyze the physics of sound in this audio clip and its emotional impact."
```

Output:

```
Domains: [physics.acoustics, emotion.auditory, audio.analysis]
Prefixes: [2001:db8:physics:acoustics::/64,
2001:db8:emotion:auditory::/64,
2001:db8:audio:analysis::/64]
```

---

## 4. Layer 2: IPv6 Bloom-Filter Routing

### 4.1 Purpose

Given the prefixes from Layer 1, the routing layer:

1. Filters candidate experts using Bloom filters
2. Sends multicast queries to relevant groups
3. Collects responses via unicast

### 4.2 Bloom-Filter Hierarchy

Each level holds a Bloom filter over the prefixes of the next level:

```
Level 1: Area prefixes (10⁷ entries, 12.5 MB, 0.8% FP)
Level 2: Domain prefixes (10⁷ entries, 12.5 MB, 0.8% FP)
Level 3: Sub-domain prefixes(10⁷ entries, 12.5 MB, 0.8% FP)
Level 4: Expert IDs (10⁷ entries, 12.5 MB, 0.8% FP)
Level 5: Metadata (10⁷ entries, 12.5 MB, 0.8% FP)
```

### 4.3 Routing Algorithm

Input: query q, hierarchy H = {B_1, ..., B_L}
Output: candidate set C

1: C ← {all area prefixes}
2: for l = 1 to L do
3: C' ← ∅
4: for each c ∈ C do
5: if B_l.contains(hash(c)) then
6: C' ← C' ∪ expand(c)
7: C ← C'
8: return C

### 4.4 Multicast Discovery

Queries are sent to IPv6 multicast groups:

```
ff0e:2001:db8:physics:acoustics::1
```

Only experts registered to this group receive the query.

### 4.5 Anycast Load Balancing

Popular experts share an anycast address:

```
2001:db8:physics:acoustics:popular::1
```


The network routes to the nearest instance.

---

## 5. Layer 3: Experts

### 5.1 Specification

- **Model**: 2B-parameter LLM (Gemma-2-2B, Qwen-2.5-1.5B, Phi-2)
- **Quantization**: INT4 (~1 GB) or INT8 (~2 GB)
- **Runtime**: Docker container
- **Interface**: gRPC over IPv6
- **Hardware**: Consumer GPU (RTX 4090) or CPU

### 5.2 Lifecycle

1. Registration → Expert announces itself via multicast
2. Discovery → Router adds expert to Bloom filter
3. Query → Expert receives multicast query
4. Inference → Expert generates response (100–1000 ms)
5. Response → Expert sends unicast response
6. Reward → Expert receives micropayment
7. Update → Expert updates its model (optional)

### 5.3 Specialization

Experts are specialized via:

- **Fine-tuning**: On domain-specific data
- **Distillation**: From a larger teacher model
- **Evolutionary training**: Via mutation and selection

---

## 6. Layer 4: Aggregator LLM

### 6.1 Purpose

The Aggregator LLM:

1. Collects expert responses
2. Weighs them by confidence and reputation
3. Synthesizes a coherent answer
4. Returns it to the user

### 6.2 Architecture

- **Model**: Initially a centralized 70B LLM
- **Interface**: gRPC
- **Weighting**: Based on expert reputation, confidence, and latency
- **Fallback**: If experts disagree, the Aggregator can query additional experts

---

## 7. Cross-Cutting Concerns

### 7.1 Security

- **Authentication**: Experts sign responses with their private key
- **Sybil resistance**: Proof-of-stake or proof-of-reputation
- **Sandboxing**: Docker containers with restricted capabilities
- **Rate limiting**: Per-expert query limits

### 7.2 Reputation

- **Initial**: All experts start with neutral reputation
- **Update**: Based on user feedback and peer review
- **Decay**: Reputation decays over time to prevent stagnation

### 7.3 Economics

- **Payment**: Micropayments via Lightning or on-chain
- **Pricing**: Determined by supply and demand
- **Rewards**: Distributed to experts proportional to contribution

### 7.4 Governance

- **Protocol**: Open, community-driven
- **Disputes**: Resolved via reputation and voting
- **Upgrades**: Via consensus of major stakeholders

---

## 8. Deployment Topology

```
┌─────────────────────────────────────────────────┐
│ Router LLM (centralized, federated later) │
└────────────────────┬────────────────────────────┘
│
┌────────────┼────────────┐
▼ ▼ ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Area 1 │ │ Area 2 │ │ Area 3 │
│ (IPv6 │ │ (IPv6 │ │ (IPv6 │
│ overlay) │ │ overlay) │ │ overlay) │
│ │ │ │ │ │
│ ┌────────┐ │ │ ┌────────┐ │ │ ┌────────┐ │
│ │Expert │ │ │ │Expert │ │ │ │Expert │ │
│ │Expert │ │ │ │Expert │ │ │ │Expert │ │
│ │Expert │ │ │ │Expert │ │ │ │Expert │ │
│ └────────┘ │ │ └────────┘ │ │ └────────┘ │
└──────────────┘ └──────────────┘ └──────────────┘
│
▼
┌─────────────────────────────────────────────────┐
│ Aggregator LLM (centralized, federated later) │
└─────────────────────────────────────────────────┘
```

---

## 9. Failure Modes

| Failure | Mitigation |
|---------|------------|
| Expert unavailable | Timeout, fallback to peer |
| Router LLM down | Federated routers via anycast |
| Aggregator LLM down | Federated aggregators |
| Network partition | Local routing within partition |
| Sybil attack | Proof-of-stake, reputation |
| Malicious expert | Reputation, sandboxing |
| Bloom-filter collision | Multi-level filtering, exact check |

---

## 10. Future Work

- **Federated Router LLMs**: Multiple routers via anycast
- **Federated Aggregator LLMs**: Multiple aggregators
- **Hierarchical experts**: Experts of different sizes
- **Cross-area routing**: Routing between areas
- **Standardization**: IETF proposal

---

## References

See `README.md` for the full reference list.
