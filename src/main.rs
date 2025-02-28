use std::env;
use std::fs;

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
    let cache_size: usize = args[2]
        .parse()
        .expect("Please provide a valid cache size (KByte)\n");
    let block_size: usize = args[3]
        .parse()
        .expect("Please provide a valid block size (Word)\n");
    let set_degree: usize = args[4]
        .parse()
        .expect("Please provide a valid set degree (1/2/4/8)\n");

    let content = fs::read_to_string(trace_file_name)
        .expect("Failed to read the file\n");

    Ok(())
}
