use rand::Rng;
use ndarray::Array2;
use std::{char::MAX, time::Instant};
use rayon::prelude::*;

const MAX_VALUE: i32 = 10000;

// fn floyd_warshall(dist: &mut Vec<Vec<i32>>) {
//     let n = dist.len();
//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 if dist[[i,k]] != MAX_VALUE && dist[[k,j]] != MAX_VALUE {
//                     dist[[i,j]] = dist[[i,j]].min(dist[[i,k]] + dist[[k,j]]);
//                 }
//             }
//         }
//     }
// }

// fn doPhase1(array: &mut Vec<Vec<i32>>) {
//     // self-dependent: this can be in-place
//     floyd_warshall( array);
// }

// fn doPhase2(array: Vec<Vec<i32>>) {


// }

// fn doPhaes3(array: Vec<Vec<i32>>) {

// }

// fn blocked_floyd_warshall(array: &mut Vec<Vec<i32>>, size: i32, block_size: i32) {
//     for round in 1..=(size/block_size) {
//         // self-dependent block
//         for k in ((round-1)*block_size+1)..=(round*block_size) {
//             doPhase1(array, k);
//         }
//     }
// }

pub fn floyd_warshall_blocked(dist: &mut Array2<i32>, block_size: usize) {
    let n = dist.len();
    let num_blocks = (n + block_size - 1) / block_size;

    for k_block in 0..num_blocks {
        let k_start = k_block * block_size;
        let k_end = std::cmp::min((k_block + 1) * block_size, n);

        println!("k_start, k_end: {}. {}", k_start, k_end);

        // Phase 1: Update the self-dependent block
        // this can't be parallelized

        for round in 1..=(n/block_size) {
            // self-dependent block
            for k in ((round-1)*block_size+1)..=(round*block_size) {
                println!("k: {}", k);
            }
            println!("new block")
        }
        for k in k_start..k_end {
            for i in k_start..k_end {
                for j in k_start..k_end {
                    if dist[[i,k]] != MAX_VALUE && dist[[k,j]] != MAX_VALUE {
                        dist[[i,j]] = std::cmp::min(dist[[i,j]], dist[[i,k]] + dist[[k,j]]);
                        // println!("i, j: {}{}", i, j)
                    }
                }
            }
        }

        // so basically in the next two loops we can have:
        // 1. Mutliple &T references for comparisons
        // 2. One (blocked?) &mut T reference to a particular block in the matrix (so we have to seperate?)

        // Phase 2: Update blocks in the same row and column as the self-dependent block
        for i_block in 0..num_blocks {
            let i_start = i_block * block_size;
            let i_end = std::cmp::min((i_block + 1) * block_size, n);

            // this can probably be parallelized
            // TODO: see any deps between cells or this row which lie in col processed below
            if i_block != k_block {
                for k in k_start..k_end {
                    for i in i_start..i_end {
                        for j in k_start..k_end {
                            if dist[[i,k]] != MAX_VALUE && dist[[k,j]] != MAX_VALUE {
                                dist[[i,j]] = std::cmp::min(dist[[i,j]], dist[[i,k]] + dist[[k,j]]);
                                println!("Row mut i, j: {}. {}", i,j);
                            }
                        }
                    }
                }
            }

            // this can probably be parallelized as well.
            // TODO: same as above
            for j_block in 0..num_blocks {
                let j_start = j_block * block_size;
                let j_end = std::cmp::min((j_block + 1) * block_size, n);

                if j_block != k_block {
                    for k in k_start..k_end {
                        for i in k_start..k_end {
                            for j in j_start..j_end {
                                if dist[[i,k]] != MAX_VALUE && dist[[k,j]] != MAX_VALUE {
                                    dist[[i,j]] = std::cmp::min(dist[[i,j]], dist[[i,k]] + dist[[k,j]]);
                                    println!("Column mut i, j: {}. {}", i,j);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Phase 3: Update all remaining blocks
        for i_block in 0..num_blocks {
            let i_start = i_block * block_size;
            let i_end = std::cmp::min((i_block + 1) * block_size, n);

            for j_block in 0..num_blocks {
                let j_start = j_block * block_size;
                let j_end = std::cmp::min((j_block + 1) * block_size, n);

                if i_block != k_block && j_block != k_block {
                    for k in k_start..k_end {
                        for i in i_start..i_end {
                            for j in j_start..j_end {
                                if dist[[i,k]] != MAX_VALUE && dist[[k,j]] != MAX_VALUE {
                                    dist[[i,j]] = std::cmp::min(dist[[i,j]], dist[[i,k]] + dist[[k,j]]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parallel_main() {
    let mut rng = rand::thread_rng();

    // Generate a random size between 1 and 10 for the square matrix
    let size = 5;

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(size);

    for _ in 0..size {
        let mut row: Vec<i32> = Vec::with_capacity(size);
        for _ in 0..size {
            // Generate a random i32 value between 0 and std::i32::MAX (inclusive)
            let value = rng.gen_range(0..MAX_VALUE);
            row.push(value);
        }
        matrix.push(row);
    }
    // println!("{:?}", matrix);
    // let mut dist = vec![
    //     vec![0, 5, std::i32::MAX, 10],
    //     vec![std::i32::MAX, 0, 3, std::i32::MAX],
    //     vec![std::i32::MAX, std::i32::MAX, 0, 1],
    //     vec![std::i32::MAX, std::i32::MAX, std::i32::MAX, 0],
    // ];

    let block_size = 10;
    let mut matrix2 = matrix.clone();
    let now = Instant::now();
    // floyd_warshall_blocked(&mut matrix, block_size);
    let elapsed = now.elapsed();
    println!("Parallel Execution Time: {:.2?}", elapsed);
    println!("{:?}", matrix);

    // println!("Parallel Shortest distances:");
    // for row in &matrix {
    //     for &val in row {
    //         if val > 10000 {
    //             print!("INF\t");
    //         } else {
    //             print!("{}\t", val);
    //         }
    //     }
    //     println!();
    // }

    // let mut dist2 = vec![
    // vec![0, 5, std::i32::MAX, 10],
    // vec![std::i32::MAX, 0, 3, std::i32::MAX],
    // vec![std::i32::MAX, std::i32::MAX, 0, 1],
    // vec![std::i32::MAX, std::i32::MAX, std::i32::MAX, 0],
    // ];
    let now = Instant::now();
    // floyd_warshall(&mut matrix2);
    let elapsed = now.elapsed();
    println!("Sequential Execution Time: {:.2?}", elapsed);
    print!("{:?}", matrix2);

    // println!("Sequential Shortest distances:");
    // for row in &matrix2 {
    //     for &val in row {
    //         if val > 10000 {
    //             print!("INF\t");
    //         } else {
    //             print!("{}\t", val);
    //         }
    //     }
    //     println!();
    // }
}