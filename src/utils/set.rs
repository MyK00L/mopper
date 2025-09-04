use crate::utils::bloom_filter::BloomFilter;
use std::hash::Hash;

pub trait Set<T>: Default + Clone {
    fn insert(&mut self, item: &T) -> bool;
    fn contains(&self, item: &T) -> bool;
}
#[derive(Clone, Default)]
pub struct AlwaysEmptySet;
impl<T> Set<T> for AlwaysEmptySet {
    fn insert(&mut self, _item: &T) -> bool {
        false
    }
    fn contains(&self, _item: &T) -> bool {
        false
    }
}

impl<T: Hash, const N: usize, const M: usize> Set<T> for BloomFilter<T, N, M> {
    fn insert(&mut self, item: &T) -> bool {
        BloomFilter::insert(self, item)
    }
    fn contains(&self, item: &T) -> bool {
        BloomFilter::contains(self, item)
    }
}
