mod lru_cache;
mod set;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lru_cache::LRUCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        5 => {},
        _ => {
            eprintln!("Usage: cargo run -- <trace_file_name> <cache_size> <block_size> <set_degree>");
            std::process::exit(1);
        },
    }

    // handle command line arguments
    let trace_file_name = &args[1];
    let cache_size: u32 = args[2]
        .parse()
        .expect("Please provide a valid cache size (KByte)\n");
    let block_size: u32 = args[3]
        .parse()
        .expect("Please provide a valid block size (Word)\n");
    let set_degree: usize = args[4]
        .parse()
        .expect("Please provide a valid set degree (1/2/4/8)\n");

    let mut lru_cache = LRUCache::new(cache_size, block_size, set_degree);

    // open file
    let file = File::open(trace_file_name).expect("Failed to open file\n");
    
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let mem_addr = line.trim();

        // remove prefix and convert hexadecimal to decimal
        let no_prefix_addr = mem_addr.trim_start_matches("0x");
        let addr_dec = u32::from_str_radix(&no_prefix_addr, 16).unwrap();
        
        lru_cache.access(addr_dec);
    }

    let miss_rate = lru_cache.count_miss_rate();
    println!("Miss Rate = {:.5}", miss_rate);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_lru_with_trace_file() {
        let file_path = "data/trace.txt";  
        let file = File::open(file_path).expect("Failed to open file");
        
        let reader = BufReader::new(file);

        // test args
        let mut cache = LRUCache::new(1, 4, 2);

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mem_addr = line.trim();
    
            let no_prefix_addr = mem_addr.trim_start_matches("0x");
            let addr_dec = u32::from_str_radix(&no_prefix_addr, 16).unwrap();
            
            cache.access(addr_dec);
        }

        println!(
            "\nTest LRU with trace.txt => hits = {}, misses = {}, miss rate = {:.5}\n",
            cache.hit, cache.miss, cache.count_miss_rate()
        );

        println!("set num = {}\n", cache.set_num);

        for (i, set) in cache.sets.iter().enumerate() {
            let list = set.debug_list();
            
            // only print non-empty sets, modify here to print all sets
            if !list.is_empty() {
                println!("Set {} => {:?}", i, list);
            }
        }
    }
}
