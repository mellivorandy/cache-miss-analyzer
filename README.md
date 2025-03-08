# cache-miss-analyzer &mdash; a high-performance Cache Simulator with LRU Policy

<br>

[<img alt="github" src="https://img.shields.io/badge/mellivorandy-cache--miss--analyzer?style=for-the-badge&logo=GitHub&label=GitHub&color=%2387CEEB" height="20">](https://github.com/mellivorandy)
[<img alt="ci" src="https://github.com/mellivorandy/cache-miss-analyzer/actions/workflows/CI.yml/badge.svg" height="20">](https://github.com/mellivorandy/cache-miss-analyzer/actions)
[<img alt="license" src="https://img.shields.io/github/license/mellivorandy/cache-miss-analyzer?style=for-the-badge&logo=GITHUB&color=light%20green" height="20">](https://github.com/mellivorandy/cache-miss-analyzer?tab=MIT-1-ov-file#readme)



<br>

This project implements a **Set-Associative Cache** simulator using an **LRU (Least Recently Used)** replacement policy. It reads a trace file containing memory addresses in hexadecimal and simulates how the cache behaves, eventually reporting the **miss rate**.

<br>

## Features

- **Set-Associative Cache**: The user specifies the total cache size (in KByte), the block size (in Words), and the set degree (N-Way).    
- **Unit Tests**: Contains a test module that can read a `trace.txt` file and print the contents of each set to debug and verify correctness.

<br>

## Overview

1. **Computing Steps and Formulae**

- Initially, convert hexadecimal memory addresses to decimal, making it easier to perform division and modulo operations.
<br><br>

- Compute the total number of cache blocks:
<br><br>
![Formula](https://latex.codecogs.com/png.latex?%5Ctext%7BTotal%20Cache%20Blocks%7D%20%3D%20%5Cfrac%7B%5Ctext%7BCache%20Size%7D%7D%7B%5Ctext%7BBlock%20Size%7D%7D)
<br><br>

- Compute the number of sets (each set holds **set\_degree** (N-Way) blocks):
<br><br>
![Formula](https://latex.codecogs.com/png.latex?\text{Number%20of%20Sets}%20=%20\frac{\text{Total%20Cache%20Blocks}}{\text{Set%20Degree}})
<br><br>

- Each block contains multiple bytes, so we determine which memory block the address belongs to:
$$
\text{Memory Block Index} = \frac{\text{Memory Address}}{\text{Block Size}}
$$
<br><br>

- Determine where in the cache this block should be mapped:
$$
\text{Set Index} = \text{Memory Block Number} \mod \text{Number of Sets}
$$
<br><br>

- Compute the tag:
$$
\text{Tag} = \frac{\text{Memory Block Number}}{\text{Number of Sets}}
$$
<br><br>

- After processing all memory accesses in trace file, compute:
$$
\text{Miss Rate} = \frac{\text{Total Misses}}{\text{Total Memory Accesses}} \times 100\%
$$
<br><br>

2. **HashMap + Doubly Linked List**  
   - **HashMap**  
     - Maps a `(tag)` inside that set to a node in the doubly linked list. 
 
     - This allows O(1) lookup to find whether a memory block is currently in the cache.  
   
   - **Doubly Linked List**  
     - Each set keeps a doubly linked list of its N blocks.  
     
     - The head represents the most recently used; the tail represents the least recently used.  
     
     - On a **hit**, the block is moved to the head; on a **miss**, if the set is full, the tail block is evicted (LRU), otherwise an empty slot or invalid block is used.

     - Why using a doubly linked list? Because when tracking usage order in the lists, removing a node from the middle or moving a node to the front are needed frequently. A doubly linked list allows you to perform the operations mentioned above in O(1). By contrast, a singly linked list requires O(n) time to find the predecessor before removal.
    
    **Dummy Head and Tail**  
    - To simplify insertions and deletions, each set maintains two **dummy nodes**, namely the **dummy head** and the **dummy tail**. The dummy head ensures that there is always a first node (most recently used item), whereas the dummy tail ensures that there is always a last node (least recently used item). 
      
    - Without dummy nodes, inserting or removing from the beginning or end of the list would require additional boundary checks.
  
3. **Valid Bit Mechanism**  
   - Originally, this project did not preallocate any invalid blocks. Instead, it simply builds a new Node on every miss if `size < capacity`. If `size == capacity` (the set is full), it **evicts** the least recently used node (via `evict()` method) before inserting the new one.

    - In other words, the design **omits** a `valid = false` state, relying on the condition `size < capacity` to detect free capacity. Each Miss either:
        - Creates a new node if not at capacity, or  
        - Evicts the oldest node if at capacity.

    - According to project requirement, checking the validation of each cache block is needed when accessing. As a result, I modify `get`, `put` and add some helper methods. If a block is `valid=false`, we can fill it without evicting another block; if all blocks are `valid=true`, we perform an LRU eviction.

    - Check out the implementation of `put` method at commit [6d00f29](https://github.com/mellivorandy/cache-miss-analyzer/commit/6d00f29e82e2023f5cf73a92751488cda5d2a2ec) at line 63.
---

## Getting Started <br><br>

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

From the project root, run

```Rust
cargo build
```

To execute the program with custom arguments, use the following command:

```Rust
cargo run -- <trace_file_name> <cache_size> <block_size> <set_degree>
```

- <trace_file_name>: The name of the trace file to be analyzed.
- <cache_size>: The total size of the cache (in KByte).
- <block_size>: The size of each block (in Words).
- <set_degree>: The associativity (number of blocks per set).

<br>

In this project structure, the trace.txt file is located in cache-miss-analyzer/data, change the path if trace file is moved or new trace files are added.

<br>

---

### Test

A `[cfg(test)]` module is included, referencing a trace.txt file for unit tests. To run them:

```Rust
cargo test -- --nocapture
```

- Change arguments at src/main.rs:71:39 for different cache configurations.
- Remove if expressions at src/main.rs:94:13 as you want to print all sets.

<br>

---

### Use Cases & Applications

- Computer Architecture Studies: Helps understand cache memory behavior and performance implications of different cache configurations.
<br><br>
- Performance Benchmarking: Useful for analyzing memory access patterns and optimizing software performance.
<br><br>

---

### Contributing

Contributions are welcome!  

1. Fork the repo and create a branch.  
2. Make changes and ensure everything works.  
3. Follow the coding style.  
4. Open a pull request with details.  

For major changes, please open an issue first.

<br>

---

### License

This project is licensed under <a href="LICENSE">MIT license</a>.
