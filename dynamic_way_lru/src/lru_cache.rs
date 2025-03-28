use crate::set::Set;

pub struct LRUCache {
    pub sets: Vec<Set>,
    pub block_size_byte: u32,
    pub set_num: u32,
    pub hit: u32,
    pub miss: u32,
}

impl LRUCache {
    pub fn new(cache_size: u32, block_size: u32, set_degree: usize) -> Self {
        let block_size_byte = block_size << 2;
        let cache_block_num = (cache_size << 10) / (block_size_byte as u32);
        let set_num = cache_block_num / (set_degree as u32);

        let sets = (0..set_num)
            .map(|_| {
                let mut set = Set::new();
                set.capacity = set_degree;
                set
            })
            .collect::<Vec<_>>();

        LRUCache {
            sets,
            block_size_byte,
            set_num,
            hit: 0,
            miss: 0,
        }
    }

    pub fn access(&mut self, address: u32) {
        let memory_block = address / self.block_size_byte;
        let set_index = memory_block % self.set_num;
        let tag = memory_block / self.set_num;

        let set = &mut self.sets[set_index as usize];

        if set.get(tag) {
            // Hit
            self.hit += 1;
        } else {
            // Miss
            self.miss += 1;
            set.put(tag);
        }
    }

    pub fn count_miss_rate(&self) -> f64 {
        let total = self.hit + self.miss;
        if total == 0 {
            0.0
        } else {
            self.miss as f64 / total as f64
        }
    }
}
