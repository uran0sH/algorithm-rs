use bitvec::prelude::*;
use std::hash::{Hash, Hasher};
use wyhash::WyHash;

pub struct BloomFilter {
    bitmap: BitVec<u16, Lsb0>,
    seeds: Vec<u64>,
}

impl BloomFilter {
    pub fn new() -> Self {
        BloomFilter {
            bitmap: bitarr![u16, Lsb0; 0; u16::MAX as usize].to_bitvec(),
            seeds: vec![3, 5, 7, 11, 13, 31, 37, 61],
        }
    }

    pub fn add(&mut self, value: String) {
        for seed in self.seeds.iter() {
            let mut hash = WyHash::with_seed(*seed);
            hash.write(value.as_bytes());
            let ret = hash.finish() as u16;
            self.bitmap.set(ret as usize, true);
        }
    }

    pub fn contains(&mut self, value: String) -> bool {
        for seed in self.seeds.iter() {
            let mut hash = WyHash::with_seed(*seed);
            hash.write(value.as_bytes());
            let ret = hash.finish() as u16;
            if !self.bitmap.get(ret as usize).unwrap() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use super::BloomFilter;

    #[test]
    fn test() {
        let a = "abc";
        let b = "bcd";
        let mut bf = BloomFilter::new();
        bf.add(a.to_string());
        bf.add(b.to_string());
        assert_eq!(true, bf.contains(a.to_string()));
        assert_eq!(true, bf.contains(b.to_string()));
    }
}
