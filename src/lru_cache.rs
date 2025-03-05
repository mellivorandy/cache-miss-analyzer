use crate::set::Set;

pub struct LRUCache {
    sets: Vec<Set>,
    block_size_byte: u32,
    set_num: u32,
    hit: u32,
    miss: u32,
}

impl LRUCache {
    pub fn new(cache_size: u32, block_size: u32, set_degree: u32) -> Self {
        let block_size_byte = block_size << 2;
        let cache_block_num = (cache_size << 10) / (block_size_byte as u32);
        let set_num = cache_block_num / (set_degree as u32);
        
        let sets = (0..set_num)
            .map(|_| Set::new(set_degree))
            .collect::<Vec<_>>();

        LRUCache {
            sets,
            block_size_byte,
            set_num,
            hit: 0,
            miss: 0,
        }
    }

    pub fn access(&mut self,  address: u32) {
        
    }

    pub fn count_miss_rate(&self) -> f64 {
        
    }
}
