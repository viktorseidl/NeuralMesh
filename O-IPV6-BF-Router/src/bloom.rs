//  ----------------------------------------------------
//          Blocked Bloom Filter
//  ----------------------------------------------------     
//
// the bit array is divided into 64-byte blocks 
// every element maps to exactly one block, 
// then sets "k" bits within it.

use xxhash_rust::xxh3::xxh3_64_with_seed;

// one CL = 64 bytes = 8 u64 words = 512 bits.
const BLOCK_WORDS: usize = 8;
const BLOCK_BITS: usize = BLOCK_WORDS * 64;

// bits per element (10 → ~0.8% false-positive rate).
pub const DEFAULT_BITS_PER_ELEMENT: usize = 10;

// number of hash functions (should beoptimal for 10 bits/element).
pub const DEFAULT_NUM_HASHES: u32 = 10;
 
#[derive(Clone)]
pub struct BlockedBloomFilter {
    blocks: Box<[u64]>,
    num_blocks: usize,
    block_mask: u64,
    num_hashes: u32,
}

impl BlockedBloomFilter {
    // Create filter sized for `n` elements.
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        Self::with_params(n, DEFAULT_BITS_PER_ELEMENT, DEFAULT_NUM_HASHES)
    }

    // Create filter with params.
    pub fn with_params(n: usize, bits_per_element: usize, num_hashes: u32) -> Self {
        let total_bits = n.saturating_mul(bits_per_element);
        let raw_blocks = (total_bits + BLOCK_BITS - 1) / BLOCK_BITS;
        let num_blocks = raw_blocks.next_power_of_two().max(1);

        Self {
            blocks: vec![0u64; num_blocks * BLOCK_WORDS].into_boxed_slice(),
            num_blocks,
            block_mask: (num_blocks - 1) as u64,
            num_hashes,
        }
    }

    // insert hash.
    #[inline(always)]
    pub fn insert_hash(&mut self, hash: u64) {
        let (block_idx, bits) = self.split_hash(hash);
        let block = &mut self.blocks[block_idx * BLOCK_WORDS..(block_idx + 1) * BLOCK_WORDS];

        // double hashing and derive 'k' probes 
        for i in 0..self.num_hashes {
            let probe = bits.wrapping_mul((i as u64) + 1);
            let word = ((probe >> 6) as usize) & (BLOCK_WORDS - 1);
            let offset = (probe & 63) as u32;
            block[word] |= 1u64 << offset;
        }
    }

    // query hash.
    #[inline(always)]
    pub fn contains_hash(&self, hash: u64) -> bool {
        let (block_idx, bits) = self.split_hash(hash);
        let block = &self.blocks[block_idx * BLOCK_WORDS..(block_idx + 1) * BLOCK_WORDS];

        // early exit on zero bit
        for i in 0..self.num_hashes {
            let probe = bits.wrapping_mul((i as u64) + 1);
            let word = ((probe >> 6) as usize) & (BLOCK_WORDS - 1);
            let offset = (probe & 63) as u32;
            if block[word] & (1u64 << offset) == 0 {
                return false;
            }
        }
        true
    }

    // split hash into: block index and block bits
    #[inline(always)]
    fn split_hash(&self, hash: u64) -> (usize, u64) {
        let block_idx = (hash & self.block_mask) as usize;
        let bits = hash
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(17);
        (block_idx, bits)
    }

    // approximate memory to use
    #[inline]
    pub fn memory_bytes(&self) -> usize {
        self.blocks.len() * 8
    }

    // number of blocks.
    #[inline]
    pub fn num_blocks(&self) -> usize {
        self.num_blocks
    }

    // expected false-positive rate.
    pub fn false_positive_rate(&self, n: usize) -> f64 {
        let m = (self.num_blocks * BLOCK_BITS) as f64;
        let k = self.num_hashes as f64;
        let n = n as f64;
        (1.0 - (-k * n / m).exp()).powf(k)
    }
}

//  warning
const _: usize = BLOCK_BITS;

// hash -> byte slice -> xxh3.
#[inline(always)]
pub fn hash_bytes(data: &[u8], seed: u64) -> u64 {
    xxh3_64_with_seed(data, seed)
}