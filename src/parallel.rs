// use std::process::exit;

// use ndarray::Array2;
use ndarray::prelude::*;
use rayon::prelude::*;

// the values need not match the sequential version.
// authors of the paper have established the correctness
// of the blocked version
pub fn floyd_warshall_blocked(array: &mut Array2<i32>, n: usize) {
    for k in 0..n {
        let row_k = array.slice(s![k, ..]).to_owned();
        // println!("rowk : {:?}", row_k);
        array.axis_iter_mut(Axis(0))
            .into_par_iter()
            .for_each(|mut row_i| {
                // println!("k = {} \t rowi : {:?}", k, row_i);
                let m_ik = row_i[k];
                // println!("mik{:?}", m_ik);
                row_i.iter_mut().zip(row_k.iter()).for_each(|(m_ij, m_kj)| {
                    // println!("mij, mkj = {}, {}", m_ij, m_kj);
                    let d_ijk = m_ik + *m_kj;
                    if d_ijk < *m_ij {
                        *m_ij = d_ijk;
                    }
                });
                // exit(0);
            })
    }
}

pub fn floyd_warshall_blocked_v2(array: &mut Array2<i32>, n: usize) {
    for k in 0..n {
        let row_k = array.slice(s![k, ..]).to_owned();
        let col_k = array.slice(s![.., k]).to_owned();
        array.axis_iter_mut(Axis(0))
            .into_par_iter()
            .enumerate()
            .for_each(|(i, mut row_i)| {
                row_i.axis_iter_mut(Axis(0)).enumerate().for_each(
                    |(j, mut m_ij)| {
                        let d_ijk = col_k[i] + row_k[j];
                        m_ij.map_inplace(|x| {
                            if d_ijk < *x {
                                *x = d_ijk;
                            }
                        })
                    },
                )
            })
    }
}