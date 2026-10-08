//  ----------------------------------------------------
//          Router
//  ----------------------------------------------------  
// 5 levels of bloom filters + sorted lists.

use rayon::prelude::*;

use crate::bloom::{hash_bytes, BlockedBloomFilter};
use crate::prefix::{Prefix, PrefixList};

// configuration
#[derive(Clone)]
pub struct LevelConfig {
    // prefix length 
    pub prefix_len: u8,
    // expected entries
    pub expected_entries: usize,
    // hash seed
    pub seed: u64,
}


pub struct Level {
    pub config: LevelConfig,
    pub bloom: BlockedBloomFilter,
    pub prefixes: PrefixList,
}

impl Level {
    pub fn new(config: LevelConfig) -> Self {
        let bloom = BlockedBloomFilter::with_capacity(config.expected_entries);
        let prefixes = PrefixList::with_capacity(config.prefix_len, config.expected_entries);
        Self {
            config,
            bloom,
            prefixes,
        }
    }

    // insert prefix
    #[inline]
    pub fn insert(&mut self, p: &Prefix) {
        debug_assert_eq!(p.len, self.config.prefix_len);
        self.prefixes.push(p.addr);
    }

    // build BF on prefix list
    pub fn finalize(&mut self) {
        self.prefixes.finalize();
        // rebuild BF on final list.
        self.bloom = BlockedBloomFilter::with_capacity(self.prefixes.len().max(1));
        for addr in self.prefixes.iter() {
            let h = hash_bytes(addr, self.config.seed);
            self.bloom.insert_hash(h);
        }
    }

    #[inline(always)]
    pub fn bloom_contains(&self, addr: &[u8; 16]) -> bool {
        let h = hash_bytes(addr, self.config.seed);
        self.bloom.contains_hash(h)
    }

    // check against prefix
    #[inline(always)]
    pub fn contains(&self, addr: &[u8; 16]) -> bool {
        self.prefixes.contains(addr)
    }
}

// full router.
pub struct HierarchicalRouter {
    levels: Vec<Level>,
}

impl HierarchicalRouter {
    pub fn new(configs: Vec<LevelConfig>) -> Self {
        let levels = configs.into_iter().map(Level::new).collect();
        Self { levels }
    }

    pub fn level_mut(&mut self, idx: usize) -> &mut Level {
        &mut self.levels[idx]
    }

    pub fn level(&self, idx: usize) -> &Level {
        &self.levels[idx]
    }

    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    // finalize 
    pub fn finalize(&mut self) {
        for level in &mut self.levels {
            level.finalize();
        }
    }

    pub fn route(&self, initial: &[Prefix]) -> Vec<Prefix> {
        let mut current: Vec<Prefix> = initial.to_vec();

        for (level_idx, level) in self.levels.iter().enumerate() {
            let target_len = level.config.prefix_len;

            // parallel filtering
            let next: Vec<Prefix> = if current.len() >= 64 {
                current
                    .par_iter()
                    .flat_map_iter(|p| {
                        Self::step_level(level, p, target_len, level_idx, self.levels.len())
                    })
                    .collect()
            } else {
                current
                    .iter()
                    .flat_map(|p| {
                        Self::step_level(level, p, target_len, level_idx, self.levels.len())
                    })
                    .collect()
            };

            current = next;
            if current.is_empty() {
                break;
            }
        }

        current
    }

    #[inline]
    fn step_level<'a>(
        level: &'a Level,
        parent: &'a Prefix,
        target_len: u8,
        level_idx: usize,
        total_levels: usize,
    ) -> Box<dyn Iterator<Item = Prefix> + 'a> {
        // if first level -> parent is start point.
        let parent_len = parent.len;

        // if parent target length then check
        if parent_len == target_len {
            if level.bloom_contains(&parent.addr) && level.contains(&parent.addr) {
                return Box::new(std::iter::once(*parent));
            }
            return Box::new(std::iter::empty());
        }

        // check children at target_len.
        let children = level.prefixes.children_of(parent);
        let is_last = level_idx + 1 == total_levels;

        Box::new(children.iter().filter_map(move |addr| {
            // Bloom filter check (fast reject).
            if !level.bloom_contains(addr) {
                return None;
            }
            // Binary search confirmation.
            if !level.contains(addr) {
                return None;
            }
            Some(Prefix {
                addr: *addr,
                len: target_len,
            })
        }))
    }

    // memory
    pub fn memory_bytes(&self) -> usize {
        self.levels
            .iter()
            .map(|l| {
                l.bloom.memory_bytes()
                    + l.prefixes.len() * 16  // 16 bytes per prefix
                    + std::mem::size_of::<Level>()
            })
            .sum()
    }
}