//! Native Rust Usage Example
//! How can you run it
//! cargo run --example main

use rust_fast_cluster::linkage;

fn main() {
    // Condensed distance matrix for 4 points (length = 6)
    let d = vec![
        0.1, // dist(0,1)
        0.2, // dist(0,2)
        0.3, // dist(0,3)
        0.4, // dist(1,2)
        0.5, // dist(1,3)
        0.6, // dist(2,3)
    ];

    let result = linkage(&d, "ward");

    println!("Linkage result (Z matrix):\n{}", result);
}
