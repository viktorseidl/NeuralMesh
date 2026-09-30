# IPv6 Addressing Schema

**Version**: 0.1.0
**Status**: Draft
**Last Updated**: 30.09.2026

---

## 1. Introduction

NeuralMesh uses IPv6 addresses to encode domain membership. This document specifies the addressing schema.

---

## 2. Address Structure

### 2.1 Overview

An IPv6 address is 128 bits. We partition it as follows:

| Global Routing Prefix | Area | Domain | Sub-Domain | Expert |
|-----|-----|-----|-----|-----|
| (48 Bit) | (16 B) | (16 Bit) | (16 Bit) | (32 Bit) |


### 2.2 Field Descriptions

| Field | Bits | Purpose |
|-------|------|---------|
| Global Routing Prefix | 48 | Standard IPv6 routing prefix |
| Area | 16 | Top-level domain (e.g., "physics") |
| Domain | 16 | Sub-domain (e.g., "acoustics") |
| Sub-Domain | 16 | Sub-sub-domain (e.g., "ultrasound") |
| Expert | 32 | Unique expert ID |

### 2.3 Example

```
2001:db8:7068:7973:6163:6f75:7374:6963
│ │ │ │ │
│ │ │ │ └─ Expert ID
│ │ │ └─ Sub-Domain ("ustic")
│ │ └─ Domain ("acou")
│ └─ Area ("phys")
└─ Global Routing Prefix
```


---

## 3. Area Allocation

### 3.1 Reserved Areas

| Area | Name | Description |
|------|------|-------------|
| 0x0000 | Reserved | Reserved for future use |
| 0x0001 | Physics | Physics-related experts |
| 0x0002 | Chemistry | Chemistry-related experts |
| 0x0003 | Biology | Biology-related experts |
| 0x0004 | Medicine | Medical experts |
| 0x0005 | Law | Legal experts |
| 0x0006 | Engineering | Engineering experts |
| 0x0007 | Arts | Arts and humanities |
| 0x0008 | Mathematics | Mathematical experts |
| 0x0009 | Language | Linguistic experts |
| 0x000A | Audio | Audio processing |
| 0x000B | Vision | Image processing |
| 0x000C | Emotion | Emotional analysis |
| ... | ... | ... |
| 0xFFFF | Open | Community-allocated |

### 3.2 Open Allocation

Areas 0x0010 to 0xFFFF are open for community allocation. Allocation is managed via the registry (see Section 6).

---

## 4. Domain Allocation

Within each area, domains are allocated:

| Domain | Name | Description |
|--------|------|-------------|
| 0x0001 | Acoustics | Sound physics |
| 0x0002 | Optics | Light physics |
| 0x0003 | Mechanics | Motion physics |
| 0x0004 | Thermodynamics | Heat physics |
| 0x0005 | Electromagnetism | EM physics |
| 0x0006 | Quantum | Quantum physics |
| ... | ... | ... |
| 0xFFFF | Open | Community-allocated |

---

## 5. Expert Allocation

### 5.1 Expert ID

The expert ID is a 32-bit integer. It is assigned by the expert itself (via hash of its public key) or by the registry.

### 5.2 Example

```
Expert ID: 0x00000001
Full address: 2001:db8:0001:0001:0001:0000:0000:0001
```

---

## 6. Registry

### 6.1 Purpose

The registry maintains:

- Area allocations
- Domain allocations
- Expert registrations
- Reputation scores

### 6.2 Implementation

- **On-chain**: For provenance, licensing, payments
- **Off-chain**: For fast lookups (IPFS, DNS)
- **Hybrid**: On-chain hash, off-chain data

### 6.3 Registration

1. Expert generates keypair
2. Expert computes ID = hash(public_key)
3. Expert registers via registry API
4. Registry adds expert to Bloom filter
5. Expert announces itself via multicast


---

## 7. Multicast Groups

### 7.1 Group Address

Multicast groups use the `ff0e::/16` prefix:

```
ff0e:<area>:<domain>:<subdomain>::1
```

### 7.2 Example

```
ff0e:0001:0001:0001::1 → All physics acoustics experts
ff0e:0004:0001:0001::1 → All medical cardiology experts
```

### 7.3 Scope

- **ff0e** = global scope
- **ff05** = site-local
- **ff02** = link-local

---

## 8. Anycast Addresses

### 8.1 Anycast Prefix

Anycast addresses use the `2001:db8:anycast::/48` prefix:

```
2001:db8:anycast:<area>:<domain>:<subdomain>:<expert>
```

### 8.2 Example

```
2001:db8:anycast:0001:0001:0001:00000001
```

### 8.3 Routing

Anycast addresses are announced via BGP:

```
BGP Announce: 2001:db8:anycast::/48
```

Traffic is routed to the nearest instance.

---

## 9. Reserved Addresses

| Address | Purpose |
|---------|---------|
| 2001:db8::/32 | Documentation (RFC 3849) |
| 2001:db8:0000::/48 | Reserved |
| 2001:db8:FFFF::/48 | Open allocation |
| ff0e::/16 | Multicast |
| 2001:db8:anycast::/48 | Anycast |

---

## 10. Future Work

- **Hierarchical allocation**: More levels
- **Dynamic allocation**: Experts can change domains
- **Cross-area routing**: Routing between areas
- **Standardization**: IETF proposal

---

## References

- RFC 8200: IPv6 Specification
- RFC 4291: IPv6 Addressing Architecture
- RFC 3849: IPv6 Documentation Prefix
- RFC 7346: IPv6 Multicast Address Scopes

See `README.md` for the full reference list.
