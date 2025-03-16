# Valid-Checking LRU &mdash; a preallocated block management approach

<br>

The Valid-Checking LRU (Least Recently Used) cache is a set-associative cache implementation that pre-allocates all cache blocks at initialization. Unlike dynamically allocated caches, this approach marks blocks as valid=false when they are not in use.

<br>

## Features
- **Pre-allocated Blocks**: All cache blocks are created at startup and stored in a fixed-size structure.

<br>

- **Valid Bit Tracking**: Each block has a valid flag to indicate whether it contains a tag.

<br>

## Implementation Details
- Each Set contains a **HashMap** (tag -> Node) and a **doubly linked list** for LRU tracking.

<br>

- Pre-allocated fixed-size cache blocks (**valid = false** initially).

<br>

- Upon accessing a block:

    1. If the block exists in the cache (Hit), it is moved to the front of the list.

    2. If it does not exist (Miss), an invalid block is reused if available.

    3. If all blocks are valid=true, the least recently used (LRU) block is evicted. Then, the new block is created and moved to the front of the list.

<br>

## Getting Started 

### Prerequisites

- [Rust](https://www.rust-lang.org/) (recommended 1.84.1 or higher)
- A terminal or command prompt to run `cargo`
<br><br>
---

### Building and Running

Clone the repository

```bash
git clone https://github.com/mellivorandy/cache-miss-analyzer.git
```

<br>

From the project root, run:

```Rust
cargo build
```

<br>

To execute the program with custom arguments, use the following command:

```Rust
cargo run -p valid_checking_lru -- <trace_file_name> <cache_size> <block_size> <set_degree>
```

- <trace_file_name>: The path of the trace file to be analyzed.
- <cache_size>: The total size of the cache (in KByte).
- <block_size>: The size of each block (in Words).
- <set_degree>: The associativity (number of blocks per set).

<br>

In this project structure, the trace.txt file is located in cache-miss-analyzer/data, change the path if the trace file is moved or new trace files are added.

<br>

Note: If the file path remains unchanged, use the path as given. Simply copy and paste the following command into your terminal:

#### Example command

```Rust
cargo run -p valid_checking_lru -- ./data/trace.txt 128 4 1
```

<br>

---

### Test

A `[cfg(test)]` test module is included, referencing a trace.txt file for unit tests. To run them:

```Rust
cargo test -p valid_checking_lru -- --nocapture
```

This prints the non-empty sets.

- Change arguments at valid_checking_lru/src/main.rs:69:39 for different cache configurations.

<br>

- Remove if expressions at valid_checking_lru/src/main.rs:92:13 as you want to print all sets.

<br>
