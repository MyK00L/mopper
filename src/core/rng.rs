/// Trait for random number generation
pub trait Rng: Clone {
    fn next_u64(&mut self) -> u64;
    fn next01(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }
}
/// The actual random number generator
#[derive(Clone, Copy)]
pub struct Splitmix64(u64);
impl Splitmix64 {
    pub fn from_u64(seed: u64) -> Self {
        Self(seed)
    }
}
impl Rng for Splitmix64 {
    fn next_u64(&mut self) -> u64 {
        let mut z = self.0.wrapping_add(0x9e3779b97f4a7c15);
        self.0 = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
