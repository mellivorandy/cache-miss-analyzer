use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lru_cache::LRUCache;

mod lru_cache;
mod set;

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
    let set_degree: u32 = args[4]
        .parse()
        .expect("Please provide a valid set degree (1/2/4/8)\n");

    let mut lru_cache = LRUCache::new(cache_size, block_size, set_degree);

    // open file and process each line
    let file = File::open(trace_file_name).expect("Failed to open the file\n");
    
    // use BufReader to improve the speed of reading trace file
    let reader = BufReader::new(file);

    // process the lines
    for line_result in reader.lines() {
        let line = line_result?;
        let mem_addr = line.trim();
        let no_prefix_addr = mem_addr.trim_start_matches("0x");
        let addr_dec = u32::from_str_radix(&no_prefix_addr, 16).unwrap();
        lru_cache.access(addr_dec);
    }

    let miss_rate = lru_cache.count_miss_rate();
    println!("Miss Rate = {:.6}", miss_rate);

    Ok(())
}
