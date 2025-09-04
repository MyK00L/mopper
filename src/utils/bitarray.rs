// using Vec<u64> instead of [u64; (N + 63) / 64] because operations on const are unstable
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitArray<const N: usize> {
    data: Vec<u64>, //[u64; (N + 63) / 64],
}
impl<const N: usize> BitArray<N> {
    pub fn new() -> Self {
        Self {
            data: vec![0; N.div_ceil(64)],
        }
    }
    pub fn set(&mut self, i: usize) {
        let (block, bit) = (i / 64, i % 64);
        self.data[block] |= 1 << bit;
    }
    pub fn get(&self, i: usize) -> bool {
        let (block, bit) = (i / 64, i % 64);
        (self.data[block] & (1 << bit)) != 0
    }
}
impl<const N: usize> Default for BitArray<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> std::fmt::Debug for BitArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..N {
            if self.get(i) {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }
        Ok(())
    }
}
impl<const N: usize> std::ops::BitOr for BitArray<N> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] | rhs.data[i];
        }
        result
    }
}
impl<const N: usize> std::ops::BitAnd for BitArray<N> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] & rhs.data[i];
        }
        result
    }
}
impl<const N: usize> std::ops::Not for BitArray<N> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..self.data.len() {
            result.data[i] = !self.data[i];
        }
        // Clear bits beyond N
        let excess_bits = self.data.len() * 64 - N;
        if excess_bits > 0 {
            let mask = (1u64 << (64 - excess_bits)) - 1;
            result.data[self.data.len() - 1] &= mask;
        }
        result
    }
}
impl<const N: usize> std::ops::BitXor for BitArray<N> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] ^ rhs.data[i];
        }
        result
    }
}
