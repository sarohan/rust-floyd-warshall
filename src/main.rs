mod sequential;
mod parallel;
mod util;

// use std::{env, process::exit};
// use clap::builder::Str;
use clap::{Arg, value_parser, Command};
use ndarray::Array2;
use crate::{parallel::floyd_warshall_blocked, sequential::sequential_main, parallel::floyd_warshall_blocked_v2};
use std::time::Instant;

fn main() {
    /*
    Asssumptions: 
    1. square matrix, everything is in powers of 2 for easy chunking
    */

     let max_weight = 100; // Maximum weight for an edge

     let matches = Command::new("Floyd-Warshall Algorithm")
     .version("1.0")
     .author("Sagar Mohan <sagarmoh@buffalo.edu")
     .about("Using command-line arguments, computes the all-pairs shortest path using the Floyd-Warshall algorithm.")
    //  .arg(arg!())
    //  .arg(
    //      Arg::new("method")
    //          .short('m')
    //          .long("method")
    //          .value_name("p or s")
    //          .help("p = Parallel; s = sequential")
    //         //  .takes_value(true)
    //          .value_parser(clap::value_parser!(char))
    //  )
     .arg(
        Arg::new("num_rows")
            .short('r')
            .long("rows")
            .value_name("INTEGER")
            .help("Specify the number of rows")
            // .takes_value(true)
            .value_parser(value_parser!(usize))
    )
    // .arg(
    //     Arg::new("num_cols")
    //         .short('c')
    //         .long("cols")
    //         .value_name("INTEGER")
    //         .help("Specify the number of cols")
    //         // .takes_value(true)
    //         .value_parser(value_parser!(usize))
    // )
    // .arg(
    //     Arg::new("block_size")
    //         .short('b')
    //         .long("block")
    //         .value_name("INTEGER")
    //         .help("Specify the block size")
    //         // .takes_value(true)
    //         .value_parser(value_parser!(usize))
    // )
     .get_matches();

    // parse the user-supplied command-line arguments
    // let method = *matches.get_one::<char>("method").expect("--method required");
    let rows: usize  = *matches.get_one("num_rows").expect("--rows required");
    // let cols: usize = *matches.get_one("num_cols").expect("--cols required");
    // let block_size :usize = *matches.get_one("block_size").expect("--blocks required");
    println!("rows: {}, cols: {}", rows, rows);

    // generate a random adjacency matrix of size [rows] x [cols]
    let mut rng = rand::thread_rng();
    let adjacency_matrix: Array2<i32> = util::create_random_adjacency_matrix(rows, rows, max_weight, &mut rng);
    let mut adjacency_matrix2 = adjacency_matrix.clone();
    let mut adjacency_matrix3 = adjacency_matrix.clone();
    // println!("Random Adjacency Matrix:\n{}", adjacency_matrix);
    println!("-----------------------");

    // let now = Instant::now();
    // let apsp: Array2<i32> = sequential_main(rows, rows, &adjacency_matrix);
    // let elapsed = now.elapsed();

    // println!("Sequential Result: \n{:?}", apsp);
    // println!("Sequential Execution Time: {:.2?}", elapsed);

    let now = Instant::now();
    floyd_warshall_blocked(&mut adjacency_matrix2, rows);
    let elapsed = now.elapsed();
    // println!("Parallel Result 1 : \n{:?}", adjacency_matrix2);
    println!("Parallel Execution Time: {:.2?}", elapsed);

    let now = Instant::now();
    floyd_warshall_blocked_v2(&mut adjacency_matrix3, rows);
    let elapsed = now.elapsed();
    // println!("Parallel Result 2: \n{:?}", adjacency_matrix3);
    println!("Parallel v2 Execution Time: {:.2?}", elapsed);


    // if method.trim() == 's' {
    //     sequential::sequential_main(&mut adjacency_matrix);
    // }
    // if *method == 'p' {
    //     parallel::parallel_main();
    // }
    // else {
    //     println!("method: '{}'", *method);
    //     println!("[error]: incorrect method supplied.");
    // }
}
