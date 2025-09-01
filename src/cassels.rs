use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use gcd::euclid_u32;
use itertools::Itertools;

use super::cyclotomic::{sin_cos_table, CyclotomicInteger};

fn skip_cyclotomic_integer(cyclotomic_integer: &CyclotomicInteger,
                           n_values: (u32, u32, u32, u32, u32)) -> bool {

    // We mostly work directly on these quantities:
    let l = &cyclotomic_integer.exponents;  // Shorthand notation
    let len = l.len();
    let (NN, N2, N3, N5, N7) = n_values;

    // Remove some cases made redundant by complex conjugation.
    if l[2] + l[len-1] > NN + l[1] {
        return true;
    }

    // Skip cases where two roots of unity differ by a factor of -1.
    for a in 0..len {
        for b in 0..a {
            if l[a] == l[b] + N2 {
                return true;
            }
        }
    }

    // Skip cases where two roots of unity differ by a factor of zeta_3.
    if N3 != 0 {
        for a in 0..len {
            for b in 0..a {
                if    l[a] == l[b] + N3
                   || l[a] == l[b] + 2*N3 {
                    return true;
                }
            }
        }
    }

    // Skip cases where three roots of unity differ by factors of zeta_5.
    if N5 != 0 {
        for a in 0..len {
            for b in 0..a {
                if     l[a] > l[b]
                    && (l[a]-l[b]) % N5 == 0 {
                    for c in 0..b {
                        if    l[b] > l[c]
                           && (l[b]-l[c]) % N5 == 0 {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // Filter for house squared <= 5.1.
    if !cyclotomic_integer.castle_strictly_less(5.1_f64) {
       return true;
    }
    
    // Skip cases visibly of form (2) of Cassels's theorem.
    if    len == 3 
       && (   l[2] == N2 - l[1]
           || l[2] == N2 + 2*l[1]
           || (2*l[2]) % NN == N2 + l[1]) {
       return true;
    }
    
    // Skip cases visibly of form (3) of Cassels's theorem.
    if     N5 != 0
        && len == 4 {
        for (i, i1, i2) in [(1,2,3), (2,1,3), (3,1,2)] {
            if    (l[i] - l[0]) % N5 == 0
               && (l[i2] - l[i1]) % N5 == 0
               && l[i] - l[0] != l[i2] - l[i1]
               && l[1] - l[0] + l[i2] - l[i1] != NN {
                return true;
            }
        }
    }

    // Skip cases where four roots of unity differ by factors of zeta_7.
    if N7 != 0 {
        for a in 0..len {
            for b in 0..a {
                if    l[a] > l[b]
                   && (l[a]-l[b]) % N7 == 0 {
                    for c in 0..b {
                        if     l[b] > l[c]
                            && (l[b]-l[c]) % N7 == 0 {
                            for d in 0..c {
                                if    l[c] > l[d]
                                   && (l[c]-l[d]) % N7 == 0 {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn loop_over_roots(N: u32, n: usize,
                       mut file_tables: &File,
                       mut file_output: &File) {
    let NN = if N%2 == 0 {N} else {2*N};
    let N2 = NN/2;
    let N3 = if NN%3 == 0 {NN/3} else {0};
    let N5 = if NN%5 == 0 {NN/5} else {0};
    let N7 = if NN%7 == 0 {NN/7} else {0};

    // Generate and output a table of cosines and signs.
    let sin_cos_table = sin_cos_table(NN);
    for j in 0..NN {
        let (sin, cos) = sin_cos_table[j as usize];
        // TODO: Would be better to output sin, cos, in that order.
        //       But one has to be very careful.
        writeln!(file_tables, "{} {} {} {}", NN, j, cos, sin).expect("output failure");
    }
    let sin_cos_table_arc = Arc::new(sin_cos_table);

    // Loop over proper divisors j_2 of NN.
    for j2 in (1..NN).filter(|x| NN % x == 0) {
        // Loop over tuples [j_3, ..., j_*] with 0 <= j_3 <= ... <= j_len < NN,
        // also requiring that gcd(j_i, NN) >= j_2.
        // The variable len is defined thereafter, and is less or equal to n.
        let (tx, rx) = mpsc::channel();
        for j3 in (0..NN).filter(|x| euclid_u32(*x, NN) >= j2) {
            let tx_clone = tx.clone();
            // Use Arc cloning to make a new reference to the tables.
            // The point is that this points to the *same* underlying memory.
            let sin_cos_table_local = Arc::clone(&sin_cos_table_arc);
            thread::spawn(move || {
                for len in 3..=n {
                    let mut exponents: Vec<u32> = vec![0; len];
                    exponents[0..3].copy_from_slice(&[0, j2, j3]);
                    for iter in (j3..NN).filter(|x| (j2 == 1) || euclid_u32(*x, NN) >= j2)
                                            .combinations_with_replacement(len-3) {
                        exponents[3..].copy_from_slice(&iter);
                        let cyclotomic_integer = CyclotomicInteger{ exponents: &exponents,
                                                                    level: NN,
                                                                    sin_cos_table: &sin_cos_table_local};
                        // Record this case in case it has not been filtered
                        if !skip_cyclotomic_integer(&cyclotomic_integer, (NN, N2, N3, N5, N7)) {
                            tx_clone.send(exponents.clone()).unwrap();
                        }
                    }
                }
                println!("Checked cases with n = {}, j_2 = {}, j_3 = {}", NN, j2, j3);
              });
          }

         // Record the level and exponents from all spawned threads
         drop(tx);
         for exponents in rx {
             println!("{:?}", exponents);
             writeln!(file_output, "{}; {:?}", NN, exponents).expect("output failure");
         }
    }
}
