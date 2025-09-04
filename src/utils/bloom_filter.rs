use crate::utils::{bitarray::BitArray, fx_hasher::FxBuildHasher};
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;

/// N is item count
/// M is bit count = -(N * ln(p)) / (ln(2)^2) where p is false positive rate
pub struct BloomFilter<T: Hash, const N: usize, const M: usize> {
    bits: BitArray<M>,
    _t: PhantomData<T>,
    hashers: [FxBuildHasher; 2],
    k_num: usize,
}
impl<T: Hash, const N: usize, const M: usize> Clone for BloomFilter<T, N, M> {
    fn clone(&self) -> Self {
        Self {
            bits: self.bits.clone(),
            _t: PhantomData,
            hashers: self.hashers,
            k_num: self.k_num,
        }
    }
}
impl<T: Hash, const N: usize, const M: usize> Default for BloomFilter<T, N, M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash, const N: usize, const M: usize> BloomFilter<T, N, M> {
    pub fn new() -> Self {
        let bits = BitArray::new();
        let hashers = [FxBuildHasher(4), FxBuildHasher(42)];
        let k_num = ((M as f64 / N as f64) * (2.0f64.ln())).round() as usize;
        Self {
            bits,
            _t: PhantomData,
            hashers,
            k_num,
        }
    }
    fn hash(&self, item: &T, hashes: &mut [u64; 2], k_i: usize) -> usize {
        if k_i < 2 {
            hashes[k_i] = self.hashers[k_i].hash_one(item);
            hashes[k_i] as usize
        } else {
            (hashes[0].wrapping_add(k_i as u64).wrapping_mul(hashes[1])) as usize
        }
    }
    pub fn insert(&mut self, item: &T) -> bool {
        let mut present = true;
        let hashes = &mut [0u64; 2];
        for i in 0..self.k_num {
            let h = self.hash(item, hashes, i);
            present &= self.bits.get(h % M);
            self.bits.set(h % M);
        }
        present
    }
    pub fn contains(&self, item: &T) -> bool {
        let hashes = &mut [0u64; 2];
        for i in 0..self.k_num {
            let h = self.hash(item, hashes, i);
            if !self.bits.get(h % M) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bloom_filter_correctness() {
        #[allow(unused)]
        const P: f64 = 0.000001; // false positive rate
        const N: usize = 1000;
        const M: usize = 28756; // ((-(N as f64) * P.ln()) / ( 2.0f64.ln() * 2.0f64.ln() )).ceil() as usize;
        let mut bf = BloomFilter::<usize, N, M>::new();
        for i in 0..N {
            assert!(
                !bf.contains(&i),
                "bf contains i: {}, might have been unlucky",
                i
            );
            assert!(!bf.insert(&i));
            assert!(bf.contains(&i));
            assert!(bf.insert(&i));
        }
        for i in 0..N {
            assert!(bf.contains(&i));
        }
        for i in N..2 * N {
            assert!(!bf.contains(&i));
        }
    }
    #[test]
    fn test_bloom_filter_false_positive_rate() {
        #[allow(unused)]
        const P: f64 = 0.01; // false positive rate
        const N: usize = 2000;
        const M: usize = 19171; // ((-(N as f64) * P.ln()) / ( 2.0f64.ln() * 2.0f64.ln() )).ceil() as usize;
        let mut bf = BloomFilter::<u32, N, M>::new();
        let mut count = 0;
        for i in 0..2000 {
            if bf.insert(&i) {
                count += 1;
            }
        }
        assert!(count < 30, "false positive count: {}", count);
        assert!(bf.contains(&42));
        assert!(!bf.contains(&4242));
    }
}
