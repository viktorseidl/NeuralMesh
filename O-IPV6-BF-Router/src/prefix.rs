//  ----------------------------------------------------
//          Prefix Handler
//  ----------------------------------------------------  
// all prefixes on a single level have same length

use std::net::Ipv6Addr;
use xxhash_rust::xxh3::xxh3_64_with_seed;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Prefix {
    pub addr: [u8; 16],
    pub len: u8,
}

impl Prefix {
    // masked prefix -> bits beyond are zeroed.
    #[inline]
    pub fn new(addr: Ipv6Addr, len: u8) -> Self {
        debug_assert!(len <= 128);
        let mut bytes = addr.octets();
        Self::mask(&mut bytes, len);
        Self { addr: bytes, len }
    }

    // parse on a string like `"2001:db8::/48"`.
    pub fn parse(s: &str) -> Result<Self, String> {
        let (addr_str, len_str) = s
            .split_once('/')
            .ok_or_else(|| format!("missing '/' in {s}"))?;
        let addr: Ipv6Addr = addr_str
            .parse()
            .map_err(|e| format!("bad address {addr_str}: {e}"))?;
        let len: u8 = len_str
            .parse()
            .map_err(|e| format!("bad prefix length {len_str}: {e}"))?;
        Ok(Self::new(addr, len))
    }

    // zero out
    #[inline]
    fn mask(bytes: &mut [u8; 16], len: u8) {
        let full = (len / 8) as usize;
        let rem = len % 8;
        if rem > 0 {
            bytes[full] &= !((1u8 << (8 - rem)) - 1);
            for b in bytes.iter_mut().skip(full + 1) {
                *b = 0;
            }
        } else {
            for b in bytes.iter_mut().skip(full) {
                *b = 0;
            }
        }
    }

    // hash prefix with xxh3
    #[inline(always)]
    pub fn hash(&self, seed: u64) -> u64 {
        let significant = ((self.len as usize) + 7) / 8;
        xxh3_64_with_seed(&self.addr[..significant.min(16)], seed)
    }

     
    #[inline]
    pub fn child_bounds(&self, target_len: u8) -> ([u8; 16], [u8; 16]) {
        debug_assert!(target_len > self.len && target_len <= 128);

        // all child bits = 0
        let lower = self.addr;

        // all child bits = 1
        let mut upper = self.addr;
        for bit in (self.len as usize)..(target_len as usize) {
            upper[bit / 8] |= 1u8 << (7 - (bit % 8));
        }

        (lower, upper)
    }

    #[inline]
    pub fn contains(&self, other: &Prefix) -> bool {
        if other.len < self.len {
            return false;
        }
        let full = (self.len / 8) as usize;
        let rem = self.len % 8;
        if self.addr[..full] != other.addr[..full] {
            return false;
        }
        if rem > 0 {
            let mask = !((1u8 << (8 - rem)) - 1);
            if (self.addr[full] & mask) != (other.addr[full] & mask) {
                return false;
            }
        }
        true
    }
}

#[derive(Clone)]
pub struct PrefixList {
    entries: Vec<[u8; 16]>,
    len: u8,
}

impl PrefixList {
    pub fn new(len: u8) -> Self {
        Self {
            entries: Vec::new(),
            len,
        }
    }

    pub fn with_capacity(len: u8, cap: usize) -> Self {
        Self {
            entries: Vec::with_capacity(cap),
            len,
        }
    }

    #[inline]
    pub fn push(&mut self, addr: [u8; 16]) {
        self.entries.push(addr);
    }

    pub fn finalize(&mut self) {
        self.entries.sort_unstable();
        self.entries.dedup();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    pub fn prefix_len(&self) -> u8 {
        self.len
    }

    // check if prefix present.
    #[inline]
    pub fn contains(&self, addr: &[u8; 16]) -> bool {
        self.entries.binary_search(addr).is_ok()
    }

    #[inline]
    pub fn children_of(&self, parent: &Prefix) -> &[[u8; 16]] {
        if self.len <= parent.len {
            return &[];
        }
        let (lower, upper) = parent.child_bounds(self.len);
        let start = self.entries.partition_point(|e| *e < lower);
        let end = self.entries.partition_point(|e| *e <= upper);
        &self.entries[start..end]
    }

    // go over entries.
    pub fn iter(&self) -> impl Iterator<Item = &[u8; 16]> {
        self.entries.iter()
    }
}