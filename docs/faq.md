# Frequently Asked Questions (FAQ)

**Version**: 0.1.0
**Status**: Draft
**Last Updated**: 30.09.2026

---

## General Questions

### What is NeuralMesh?

NeuralMesh is a decentralized Mixture-of-Experts (MoE) architecture that distributes billions of specialized 2B-parameter LLMs across a global IPv6 network. Routing is performed via hierarchical Bloom filters over IPv6 prefixes.

### Why "NeuralMesh"?

"Neural" refers to the neural networks (LLM experts). "Mesh" refers to the decentralized, interconnected network topology. Together, it evokes a nervous system for AI.

### Is this a real project?

It is a **concept** in early development. We are looking for collaborators to build a prototype.

### Who is behind this?

The project was initiated by an experienced IT specialist and application developer with 20+ years of experience in medical software, real-time communication systems, blockchain, and ML/NLP. See `README.md` for details.

---

## Technical Questions

### How is this different from DeepSeek-V3 or Mixtral?

DeepSeek-V3 and Mixtral are **centralized** MoE systems with **learned** routers. NeuralMesh is **decentralized** with **Bloom-filter** routing. Key differences:

| Aspect | DeepSeek-V3 | NeuralMesh |
|--------|-------------|------------|
| Experts | 256 | 10¹²+ |
| Routing | learned | Bloom + IPv6 |
| Hosting | central | decentralized |
| Interpretability | low | high |
| Access | closed | open |

### Why 2B parameters?

2B is a sweet spot:

- **Large enough** for coherent language and reasoning
- **Small enough** to run on consumer hardware (RTX 4090)
- **Proven** in DeepSeek-V3 (256 experts of ~2B each)

### Why IPv6?

IPv6 has:

- **2¹²⁸ addresses**: Enough for 10³⁸ experts
- **Hierarchical structure**: Directly usable as routing hierarchy
- **Native multicast**: For discovery
- **Native anycast**: For load balancing

### Why Bloom filters?

Bloom filters:

- **Fast**: O(k) lookups
- **Compact**: 12.5 MB for 10⁷ elements
- **Scalable**: Hierarchical partitioning
- **Interpretable**: Unlike learned routers

### What about false positives?

Bloom filters have false positives. We mitigate via:

- **Multiple levels**: Each level filters further
- **Exact check**: After Bloom filtering, exact check
- **Caching**: Cache results to avoid repeated filtering

### How do experts communicate?

Via gRPC over IPv6:

- **Multicast**: For discovery
- **Unicast**: For responses
- **Anycast**: For load balancing

### What hardware is needed?

For a single expert:

- **CPU**: Modern x86 or ARM
- **RAM**: 8 GB
- **GPU**: Optional (RTX 4090 recommended)
- **Storage**: 10 GB
- **Network**: IPv6-capable

### How is training done?

Initially: **Manual** fine-tuning and distillation.

Later: **Automated** via larger LLMs generating training data and evolutionary training.

---

## Economic Questions

### How are experts paid?

Via **micropayments**:

- **Lightning Network**: Off-chain, fast, cheap
- **On-chain**: For large payments
- **Reputation**: For long-term incentives

### Who pays?

Users pay for queries. Payments are distributed to experts proportional to contribution.

### How is pricing determined?

Via **supply and demand**. Popular experts can charge more. New experts charge less to attract users.

### Is this sustainable?

Yes, if:

- **Demand** for AI is high
- **Costs** are low (decentralized, no data center)
- **Payments** are efficient (Lightning)

---

## Security Questions

### How is Sybil resistance achieved?

Via:

- **Proof-of-stake**: Experts stake tokens
- **Proof-of-reputation**: Experts build reputation
- **Proof-of-work**: Experts solve puzzles

### How are malicious experts handled?

Via:

- **Reputation**: Low-reputation experts deprioritized
- **Sandboxing**: Docker containers
- **Verification**: Multi-LLM consensus for critical queries

### How is privacy protected?

Via:

- **Encryption**: Queries encrypted end-to-end
- **Anonymity**: Optional anonymous queries
- **Local inference**: Some queries processed locally

### What about regulatory compliance?

Compliance is handled at the **application layer**. The protocol itself is neutral.

---

## Community Questions

### How can I contribute?

See `README.md` for details. Priority areas:

1. IPv6 overlay setup
2. Bloom-filter routing prototype
3. Expert training
4. Documentation
5. Community growth

### Is there funding?

Currently: **No**. We are looking for:

- **Grants**: NLnet, EU Horizon, NSF
- **Sponsors**: Companies interested in decentralized AI
- **Crowdfunding**: Kickstarter, Patreon
- **Donations**: GitHub Sponsors

### When will there be a prototype?

Target: **6 months** after securing funding.

### Will this be open source?

**Yes**. MIT License. Everything is public.

### How can I stay updated?

- **GitHub**: Star and watch the repository
- **Discord / Matrix**: Join the discussion
- **Email**: Subscribe to the newsletter

---

## Philosophical Questions

### Is this AGI?

No. NeuralMesh is a **scaling architecture**, not AGI. It may enable emergent capabilities, but AGI is not guaranteed.

### Will this replace humans?

No. NeuralMesh is a **tool**. It augments human capabilities, not replaces them.

### What about AI safety?

Safety is a **core concern**. We:

- **Document** risks
- **Mitigate** via reputation, sandboxing, governance
- **Collaborate** with AI safety researchers

### What about alignment?

Alignment is **not solved** by NeuralMesh. It is a separate research problem.

### Is this dangerous?

Any powerful technology can be dangerous. We:

- **Are transparent** about risks
- **Encourage** ethical use
- **Welcome** scrutiny

---

## Miscellaneous

### What does "NeuralMesh" mean?

See "Why NeuralMesh?" above.

### How do I pronounce "NeuralMesh"?

"Ner-al Mesh" (English) or "Noi-ral Mesch" (German).

### What is the mascot?

None yet. Suggestions welcome.

### What is the color scheme?

Blue and green (suggesting neural networks and networks).

### Where can I find the logo?

Coming soon.

---

## Still Have Questions?
 
- **Email**: viktorseidl@gmail.com

---

## References

See `README.md` for the full reference list.
