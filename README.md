# 🌐 NeuralMesh

## Decentralized Mixture-of-Experts over IPv6 with Hierarchical Bloom-Filter Routing

[![Status](https://img.shields.io/badge/Status-Concept%20%2F%20Early%20Stage-orange)]()
[![License](https://img.shields.io/badge/License-MIT-blue)]()
[![Contributions](https://img.shields.io/badge/Contributions-Welcome-brightgreen)]()
[![Open Source](https://img.shields.io/badge/Open%20Source-Yes-success)]()

---

## 🚀 The Vision in One Sentence

**We are building a global network of billions of specialized AI experts, addressed via IPv6 and routed through hierarchical Bloom filters – decentralized, scalable, resilient, and open to everyone.**

Instead of a monolithic LLM in a data center: **a living network of specialized 2B models** that communicate, complement each other, and collectively achieve more than the sum of their parts.

---

## 💡 The Core Idea

Current Mixture-of-Experts (MoE) systems like DeepSeek-V3 or Mixtral are **centralized**:

- All experts reside in **one** data center
- The router is **learned** and **uninterpretable**
- Scaling is limited by **hardware**
- Access is **monopolized**

**NeuralMesh** breaks with this paradigm:

| Aspect | Classic MoE | NeuralMesh |
|--------|-------------|------------|
| **Experts** | 8–256, central | 10¹²+, decentralized |
| **Routing** | learned router | Bloom filter + IPv6 |
| **Addressing** | internal | IPv6 |
| **Hosting** | one data center | distributed network |
| **Discovery** | central | multicast |
| **Scaling** | vertical | horizontal |
| **Access** | monopolized | open |

### How Does It Work?
```
┌─────────────────────────────────────────────────┐
│ User Query: "Analyze the physics in this dog │
│ image and the emotional impact" │
└────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────┐
│ Router LLM (semantic analysis) │
│ → Categories: [Dog, Physics, Emotion, Image] │
└────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────┐
│ IPv6 Bloom-Filter Routing │
│ → Candidates: 2001:db8:dog:physics::/64 │
│ → Candidates: 2001:db8:dog:emotion::/64 │
└────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────┐
│ 2B Experts (distributed nodes, parallel) │
│ → Physics expert responds │
│ → Emotion expert responds │
│ → Image expert responds │
└────────────────────┬────────────────────────────┘
▼
┌─────────────────────────────────────────────────┐
│ Aggregator LLM (synthesis) │
│ → Combines responses into coherent output │
└─────────────────────────────────────────────────┘
```

**The key trick**: The hierarchical structure of IPv6 addresses is **directly used as a routing hierarchy**. Each expert has an address that encodes its domain membership. Bloom filters at each level filter candidates with **microsecond latency** – without a central database, without a learned router.

---

## 🎯 Why This Matters

### 1. **Scaling Without Limits**
IPv6 has 2¹²⁸ ≈ 3.4 × 10³⁸ addresses. We can **theoretically address more experts than there are atoms in the observable universe**. Practically: billions.

### 2. **Decentralization**
Anyone can host, train, and monetize an expert. No monopoly, no central control.

### 3. **Fault Tolerance**
If a node fails, the network routes to the next one. No single point of failure.

### 4. **Interpretability**
Bloom-filter routing is **traceable** – unlike learned routers.

### 5. **Economic Participation**
Micropayments for inference. Those who contribute compute power are rewarded.

### 6. **Openness**
Open source, open standards, open research. No vendor lock-in.

---

## 🛠️ Technical Foundations

### IPv6 as Routing Hierarchy
| Global Routing Prefix | Area | Domain | Sub-Domain | Expert |
|-----------------------|------|--------|------------|--------|
| (48 Bit) | (16 B) | (16 Bit) | (16 Bit) | (32 Bit) |

```
Example:
2001:db8:dog:physics:mechanics:0000:0000:0001
```

### Hierarchical Bloom-Filter Routing

Each level holds a Bloom filter over the prefixes of the next level:

- **Level 1**: 10⁷ area prefixes → 0.8% false-positive rate
- **Level 2**: 10⁷ domain prefixes → 0.8%
- **Level 3**: 10⁷ sub-domain prefixes → 0.8%
- **Level 4**: 10⁷ expert IDs → 0.8%
- **Level 5**: 10⁷ metadata → 0.8%

**Cumulative candidate set after 5 levels**: < 20 (starting from 10⁷).

### Multicast Discovery

Queries to "all physics experts in the dog area":

```
ff0e:2001:db8:dog:physics::1
```


Only registered experts receive the query. **No broadcast storm.**

### Anycast Load Balancing

Frequently queried experts share an anycast address. The network routes to the nearest instance.

---

## 🧬 The Roadmap

### Phase 1: Concept & Community (current)

- [x] Create concept paper
- [x] README and foundations
- [ ] Find collaborators
- [ ] Set up discussion forum
- [ ] Document initial designs

### Phase 2: Proof of Concept

- [ ] IPv6 overlay (Yggdrasil or CJDNS)
- [ ] Bloom-filter routing prototype
- [ ] 10–100 2B experts (Gemma-2-2B fine-tuned)
- [ ] Multicast discovery
- [ ] Router LLM (Mixtral 8x7B)
- [ ] Aggregator LLM

**Budget**: ~€50,000 (sponsors needed) | **Duration**: ~6 months

### Phase 3: Empirical Evaluation

- [ ] Latency measurements
- [ ] False-positive rates
- [ ] Scalability tests
- [ ] Write paper (SIGCOMM / MLSys / NeurIPS)

### Phase 4: Standardization

- [ ] IETF proposal for IPv6 LLM addressing
- [ ] Bloom-filter routing protocol
- [ ] Multicast discovery protocol
- [ ] Micropayment protocol

### Phase 5: Production

- [ ] Economic model
- [ ] Reputation system
- [ ] Sybil resistance
- [ ] Governance

---

## 👤 About the Initiator

I am a **IT specialist and application developer with over 20 years of experience**.

**My focus areas:**

- 🏥 **Medical software applications** – secure, reliable, regulated
- ⚡ **Real-time communication systems** – latency, scaling, fault tolerance
- ⛓️ **Blockchain** – decentralization, consensus, smart contracts
- 🧠 **Machine Learning & NLP** – for 6 years, with a focus on practical applications

**Why I am starting this project:**

Throughout my career I have built many systems that needed to **scale**. Real-time communication, medical data, decentralized networks. I learned: **The best solutions are decentralized, open, and hierarchically structured.**

NeuralMesh is the synthesis of these experiences. It is **not an academic thought experiment** – it is a **concrete architecture** for a problem that must be solved **now**: How do we scale AI without monopolizing it?

---

## 🤝 I Am Looking for Collaborators

This project is **too big for one person**. I am looking for **volunteers** who **want to build something new**.

### Who I Am Looking For

- 🧠 **AI researchers** – MoE, routing, training, alignment
- 🌐 **Network experts** – IPv6, multicast, routing protocols
- 🔧 **Systems engineers** – Docker, Kubernetes, distributed systems
- ⛓️ **Blockchain developers** – micropayments, reputation, governance
- 📝 **Tech writers** – documentation, papers, community
- 🎨 **Designers** – UI/UX for decentralized systems
- 🧪 **Curious minds** – who simply want to think and build along

### What I Offer

- **Open communication** – everything is publicly documented
- **Flat hierarchy** – every contribution counts
- **Fair recognition** – authorship, credits, potentially later monetization
- **Learning opportunity** – we are building something that does not exist yet
- **Fun** – really. This will be exciting.

### How You Can Join

1. **⭐ Star** this repository – shows interest
2. **🐛 Open an issue** – ask questions, make suggestions, point out contradictions
3. **💬 Discuss** – in the issues or in the [discussion forum]
4. **🔧 Contribute code** – pull requests welcome
5. **📢 Share** – tell your network about it

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Concept Paper](docs/concept.md) | Detailed technical description |
| [Workflow-Concept](docs/workflow.md) | Frequently asked questions |
| [Architecture](docs/architecture.md) | Detailed system architecture |
| [Routing](docs/routing.md) | Bloom-filter routing in detail |
| [IPv6 Schema](docs/ipv6-schema.md) | Addressing hierarchy |
| [Roadmap](docs/roadmap.md) | Detailed roadmap |
| [FAQ](docs/faq.md) | Frequently asked questions |

---

## 🧪 Related Work

We stand on the shoulders of giants:

- **Mixture-of-Experts**: Jacobs et al. (1991), Shazeer et al. (2017), Fedus et al. (2022)
- **DeepSeek-V3**: 671B parameters, 256 experts, top-k routing
- **Mixtral 8x7B**: 46B parameters, 8 experts, top-2 routing
- **Bloom Filters**: Bloom (1970), Broder & Mitzenmacher (2004)
- **IPv6**: RFC 8200, RFC 4291
- **Decentralized AI**: Petals, federated learning

**Our contribution**: The **combination** of decentralized MoE, IPv6 addressing, hierarchical Bloom-filter routing, and multicast discovery.

---

## ⚠️ Honest Assessment

We are **honest**:

- **This is a concept** – not a finished product.
- **We have no prototype** – not yet.
- **There are open questions** – training, quality, economics, security.
- **It may fail** – like any ambitious project.

**But**: The idea is **well-founded**, the building blocks are **proven**, and the need is **real**.

If you **want to build something that does not exist yet**, you are in the right place.

---

## 📜 License

[MIT License](LICENSE) – free to use, modify, share.

---

## 📬 Contact

- **GitHub Issues**: for technical discussions 
- **Email**: viktorseidl@gmail.com
- **Matrix/Discord**: ferran87_nfm

---

## 🙏 Thank You

Thank you for reading this far. If the idea **even slightly** excites you – **join in**. We need **everyone** who thinks, builds, and doubts with us.

**Together we are building the nervous system for the next generation of AI.**

---

<p align="center">
  <strong>⭐ If you like the idea, give us a star – it helps others find us.</strong>
</p>

<p align="center">
  <em>"The best systems are decentralized, open, and hierarchically structured."</em>
</p>

---

## Appendix: Glossary

| Term | Meaning |
|------|---------|
| **MoE** | Mixture-of-Experts – architecture with specialized sub-models |
| **Expert** | A 2B-parameter LLM, specialized in a domain |
| **Router LLM** | Model that analyzes queries and selects experts |
| **Aggregator LLM** | Model that combines expert responses |
| **Bloom Filter** | Probabilistic data structure for set membership |
| **IPv6** | Internet Protocol Version 6 – 128-bit addresses |
| **Multicast** | Network communication to a group |
| **Anycast** | Network communication to the nearest node |
| **False Positive** | A Bloom filter says "contained" although it is not |
| **Yggdrasil** | Decentralized IPv6 overlay network |

---

**Version**: 0.1.0 (Concept Phase)
**Last Updated**: 30.09.2026
