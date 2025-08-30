use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use gcd::euclid_u32;
use itertools::Itertools;

use super::cyclotomic::{sin_cos_table, CyclotomicIntegerExponents};

pub fn loop_over_roots(n0: u32, max_len: usize, mut file_tables: &File, mut file_output: &File) {
    let n  = if n0%2 == 0 {n0} else {2*n0};
    let n2 = n/2;
    let n3 = if n%3 == 0 {n/3} else {0};
    let n5 = if n%5 == 0 {n/5} else {0};
    let n7 = if n%7 == 0 {n/7} else {0};

    // Generate and output a table of cosines and signs.
    let sin_cos_table = sin_cos_table(n);
    for j in 0..n {
        let (sin, cos) = sin_cos_table[j as usize];
        // TODO: Would be better to output sin, cos, in that order.
        //       But one has to be very careful.
        write!(file_tables, "{} {} {} {}\n", n, j, cos, sin).expect("output failure");
    }
    let sin_cos_table_arc = Arc::new(sin_cos_table);

    // Loop over proper divisors j_2 of n.
    for j2 in (1..n).filter(|x| n % x == 0) {
        // Loop over tuples [j_3, ..., j_*] with 0 <= j_3 <= ... <= j_* <= n,
        // also requiring that gcd(j_i, n) >= j_2 and j_3 < n.
        // Note: we allow n as an exponent as a proxy for a zero summand.
        let (tx, rx) = mpsc::channel();
        for j3 in (0..n).filter(|x| euclid_u32(*x, n) >= j2) {
            let tx_clone = tx.clone();
            // Use Arc cloning to make a new reference to the tables.
            // The point is that this points to the *same* underlying memory.
            let sin_cos_table_local = Arc::clone(&sin_cos_table_arc);
            thread::spawn(move || {

                for len in (3..=max_len) {
                    'skipping_cases:
                    for iter in (j3..n).filter(|x| (j2 == 1) || euclid_u32(*x, n) >= j2)
                                       .combinations_with_replacement(len-3) {

                        let l: Vec<u32> = vec![0, j2, j3].into_iter().chain(iter).collect();

                        // Remove some cases made redundant by complex conjugation.
                        if l[2] + l[len-1] > n + l[1] {
                            continue 'skipping_cases;
                        }

                        // Skip cases where two roots of unity differ by a factor of -1.
                        for a in 0..len {
                            for b in 0..a {
                                if l[a] == l[b] + n2 {
                                    continue 'skipping_cases;
                                }
                            }
                        }

                        // Skip cases where two roots of unity differ by a factor of zeta_3.
                        if n3 != 0 {
                            for a in 0..len {
                                for b in 0..a {
                                    if    l[a] == l[b] + n3
                                       || l[a] == l[b] + 2*n3 {
                                        continue 'skipping_cases;
                                    }
                                }
                            }
                        }

                        // Skip cases where three roots of unity differ by factors of zeta_5.
                        if n5 != 0 {
                            for a in 0..len {
                                for b in 0..a {
                                    if     l[a] > l[b]
                                        && (l[a]-l[b]) % n5 == 0 {
                                        for c in 0..b {
                                            if    l[b] > l[c]
                                               && (l[b]-l[c]) % n5 == 0 {
                                                continue 'skipping_cases;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Filter for house squared <= 5.1.
                        let ex = CyclotomicIntegerExponents{ exponents: &l,
                                                             level: n,
                                                             sin_cos_table: &sin_cos_table_local};
                        if !ex.compare_house_squared(5.1 as f64) {
                           continue 'skipping_cases;
                        }
                        
                        // Skip cases visibly of form (2) of Cassels's theorem.
                        if len == 3 {
                            if    l[2] == n/2 - l[1]
                               || l[2] == n/2 + 2*l[1]
                               || (2*l[2]) % n == n/2 + l[1] {
                                continue 'skipping_cases;
                            }
                        }
                        
                        // Skip cases visibly of form (3) of Cassels's theorem.
                        if     n5 != 0
                            && len == 4 {
                            for (i, i1, i2) in [(1,2,3), (2,1,3), (3,1,2)] {
                                if    (l[i] - l[0]) % n5 == 0
                                   && (l[i2] - l[i1]) % n5 == 0
                                   && l[i] - l[0] != l[i2] - l[i1]
                                   && l[1] - l[0] + l[i2] - l[i1] != n {
                                    continue 'skipping_cases;
                                }
                            }
                        }

                        // Skip cases where four roots of unity differ by factors of zeta_7.
                        if n7 != 0 {
                            for a in 0..len {
                                for b in 0..a {
                                    if    l[a] > l[b]
                                       && (l[a]-l[b]) % n7 == 0 {
                                        for c in 0..b {
                                            if     l[b] > l[c]
                                                && (l[b]-l[c]) % n7 == 0 {
                                                for d in 0..c {
                                                    if    l[c] > l[d]
                                                       && (l[c]-l[d]) % n7 == 0 {
                                                        continue 'skipping_cases;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Record this case
                        tx_clone.send(l.clone()).unwrap();

                    }
                }

                println!("Checked cases with n = {}, j_2 = {}, j_3 = {}", n, j2, j3);
              });
          }

         // Record the level and exponents from all spawned threads
         drop(tx);
         for l in rx {
             println!("{:?}", l);
             write!(file_output, "{}; {:?}\n", n, l).expect("output failure");
         }
    }

}
