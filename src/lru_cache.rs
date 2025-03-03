use crate::set::Set;

struct LRUCache {
    sets: Vec<Set>,
    hit: u32,
    miss: u32,
}

impl LRUCache {
    fn new(set_num: u32, set_degree: u32) -> Self {
        let sets = (0..set_num)
            .map(|_| Set::new(set_degree))
            .collect::<Vec<_>>();

        LRUCache {
            sets,
            hit: 0,
            miss: 0,
        }
    }

    fn access() {

    }

    fn count_miss_rate() {
        
    }
}
