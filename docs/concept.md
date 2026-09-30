# Concept Paper: NeuralMesh

## Decentralized Mixture-of-Experts over IPv6 with Hierarchical Bloom-Filter Routing

**Version**: 0.1.0
**Status**: Draft / Concept
**Last Updated**: 30.09.2026

---

## 1. Abstract

NeuralMesh is a decentralized architecture for Mixture-of-Experts (MoE) systems that leverages IPv6 addressing and hierarchical Bloom-filter routing. Unlike centralized MoE systems (DeepSeek-V3, Mixtral), which rely on learned routers within a single data center, NeuralMesh distributes billions of specialized 2B-parameter LLMs across a global IPv6 network. Each expert receives a unique, hierarchically structured IPv6 address encoding its domain membership. Routing is performed through cascaded Bloom filters over IPv6 prefixes, augmented by multicast discovery and anycast load balancing.

This document describes the motivation, architecture, and open challenges of NeuralMesh. It is a **conceptual contribution**; empirical validation is pending.

---

## 2. Motivation

### 2.1 The Scaling Problem

Large Language Models (LLMs) have grown from GPT-2 (1.5B) to DeepSeek-V3 (671B) in five years. This growth has produced emergent capabilities, but also:

- **Centralization**: Training and inference require massive data centers.
- **Monopolization**: Only a few organizations can afford frontier models.
- **Brittleness**: A single point of failure.
- **Opacity**: Learned routers are not interpretable.
- **Cost**: Inference costs scale linearly with active parameters.

### 2.2 Mixture-of-Experts as a Partial Solution

MoE reduces inference cost by activating only a subset of parameters per token. DeepSeek-V3 activates 37B of 671B parameters per token. This is efficient, but still centralized.

### 2.3 The NeuralMesh Proposal

We propose to **decentralize MoE** entirely:

- **Experts** are distributed across a global IPv6 network.
- **Routing** uses Bloom filters over IPv6 prefixes.
- **Discovery** uses IPv6 multicast.
- **Load balancing** uses IPv6 anycast.
- **Addressing** uses the hierarchical structure of IPv6.

This yields a system that is:

- **Scalable** to 10¹²+ experts
- **Decentralized** (no central authority)
- **Fault-tolerant** (no single point of failure)
- **Interpretable** (Bloom-filter routing is traceable)
- **Open** (anyone can host an expert)

---

## 3. Core Concepts

### 3.1 Experts

An expert is a **2B-parameter LLM**, specialized in a narrow domain (e.g., "physics of sound", "veterinary oncology", "medieval Latin poetry"). Experts are:

- **Trained** independently (via fine-tuning or distillation)
- **Hosted** on distributed nodes (Docker containers)
- **Addressed** via IPv6
- **Discovered** via multicast
- **Routed** via Bloom filters

### 3.2 Router LLM

A **Router LLM** performs semantic analysis of user queries and selects the relevant area prefixes. It is a centralized component (initially) and a potential bottleneck.

### 3.3 Aggregator LLM

An **Aggregator LLM** combines expert responses into a coherent answer. Like the Router LLM, it is centralized initially.

### 3.4 IPv6 as Routing Hierarchy

IPv6 addresses are structured hierarchically:
