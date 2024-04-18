// use crate::util;
use std::cmp::min;
use ndarray::Array2;

pub fn sequential_main(row_num: usize, col_num: usize, array: & Array2<i32>) -> Array2<i32>{
    // let mut distance_matrix = util::create_distance_matrix(row_num, col_num, 999);
    let mut distance_matrix = array.clone();
    // let mut i: i32 = 0;
    // let mut j: i32 = 0;
    for  i in 0..row_num {
        for j in 0..col_num {
            distance_matrix[[i,j]] = array[[i,j]] 
        }
    }

    for i in 0..row_num {
        // for j in 0..col_num {
        distance_matrix[[i,i]] = 0;
    }

    for k in 0..row_num {
        for i in 0..row_num {
            for j in 0..col_num {
                    distance_matrix[[i,j]] = min(distance_matrix[[i,j]], distance_matrix[[i,k]] + distance_matrix[[k,j]]);
            }
        }
    }
    distance_matrix
}
