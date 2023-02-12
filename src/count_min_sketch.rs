use std::time::SystemTime;

use rand::{Rng, SeedableRng};

const CM_DEPTH: usize = 4;

/// Count-Min Sketch
#[derive(Debug)]
struct CmSketch {
    rows: Vec<CmRow>,
    seed: Vec<usize>,
    mask: usize,
}

impl CmSketch {
    pub fn new(num_counters: usize) -> Self {
        if num_counters == 0 {
            panic!("bad number counters");
        }
        let counters = next_2_power(num_counters);
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        Self {
            rows: vec![CmRow::new(num_counters); CM_DEPTH],
            seed: vec![rng.gen::<usize>(); CM_DEPTH],
            mask: counters - 1,
        }
    }

    pub fn increment(&mut self, hashed: usize) {
        for (i, row) in self.rows.iter_mut().enumerate() {
            row.increment((hashed ^ self.seed[i]) & self.mask);
        }
    }

    pub fn estimate(&self, hashed: usize) -> usize {
        let mut mi = 255_u8;
        for (i, row) in self.rows.iter().enumerate() {
            let val = row.get((hashed ^ self.seed[i]) & self.mask);
            mi = std::cmp::min(mi, val);
        }
        mi as usize
    }

    pub fn reset(&mut self) {
        for row in self.rows.iter_mut() {
            row.reset()
        }
    }

    pub fn clear(&mut self) {
        for row in self.rows.iter_mut() {
            row.clear()
        }
    }
}

#[derive(Debug, Clone)]
struct CmRow(Vec<u8>);

impl CmRow {
    pub fn new(num_counters: usize) -> Self {
        Self(vec![0u8; num_counters / 2])
    }

    pub fn get(&self, n: usize) -> u8 {
        (self.0[n / 2] >> ((n & 1) * 4)) & 0x0f
    }

    pub fn increment(&mut self, n: usize) {
        let i = n / 2;
        let s = (n & 1) * 4;
        let v = (self.0[i] >> s) & 0x0f;
        if v < 15 {
            self.0[i] += 1 << s;
        }
    }

    pub fn reset(&mut self) {
        for i in self.0.iter_mut() {
            *i >>= 1;
            *i &= 0x77;
        }
    }

    pub fn clear(&mut self) {
        for i in self.0.iter_mut() {
            *i = 0;
        }
    }
}

pub fn next_2_power(mut x: usize) -> usize {
    let mut count = 0;
    while x != 0 {
        x >>= 1;
        count += 1;
    }
    1 << count
}

#[cfg(test)]
mod test {
    use crate::count_min_sketch::next_2_power;

    use super::CmRow;

    #[test]
    fn test_cm_row() {
        let mut cm_row = CmRow::new(4);
        assert_eq!(vec![0, 0], cm_row.0);
        cm_row.increment(1);
        assert_eq!(vec![16, 0], cm_row.0);
        cm_row.increment(1);
        assert_eq!(vec![32, 0], cm_row.0);
        cm_row.reset();
        assert_eq!(vec![16, 0], cm_row.0);
        cm_row.increment(0);
        assert_eq!(vec![17, 0], cm_row.0);
    }

    #[test]
    fn test_next_2_power() {
        assert_eq!(2, next_2_power(1));
        assert_eq!(8, next_2_power(5));
    }
}
