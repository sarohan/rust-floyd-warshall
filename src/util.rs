use ndarray::Array2;
use rand::rngs::ThreadRng;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub fn create_random_adjacency_matrix(rows: usize, cols: usize, max_weight: i32, rng: &mut ThreadRng) -> Array2<i32> {
    // Create an adjacency matrix with random weights
    let mut adjacency_matrix: Array2<i32> = Array2::random_using((rows, cols), Uniform::new(0, max_weight + 1), rng);

    // Set the diagonal to 0 to indicate no self-loops
    for i in 0..rows.min(cols) {
        adjacency_matrix[(i, i)] = 0;
    }

    adjacency_matrix
}

// #[cfg(debug_assertions)]
// pub fn create_distance_matrix(rows: usize, cols: usize, value: i32) -> Array2<i32> {
//     // Create a 2D array filled with the specified value
//     let matrix: Array2<i32> = Array2::from_elem((rows, cols), value);
//     matrix
// }