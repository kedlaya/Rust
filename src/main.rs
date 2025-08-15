mod cyclotomic;

use cyclotomic::{cosine_sine_table, CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use itertools::Itertools;

use gcd::euclid_u32;

fn loop_over_roots(n0: u32, len: usize, mut f1: &File, mut f2: &File) {
    let n = if n0%2 == 0 {n0} else {2*n0};
    let n2 = n / 2;
    let n3 = if n%3 == 0 {n / 3} else {0};
    let n5 = if n%5 == 0 {n / 5} else {0};
    let n7 = if n%7 == 0 {n / 7} else {0};

    // Generate and output a table of cosines and signs.
    let (cos_table, sin_table) = cosine_sine_table(n);
    for j in 0..n {
        write!(f1, "{} {} {} {}\n", n, j, cos_table[j as usize], sin_table[j as usize]).expect("output failure");
    }
    let cos_table_Arc = Arc::new(cos_table);
    let sin_table_Arc = Arc::new(sin_table);

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
           let cos_table_local = Arc::clone(&cos_table_Arc);
           let sin_table_local = Arc::clone(&sin_table_Arc);
           thread::spawn(move || {
               let mut l: Vec<u32> = vec![0; len];
               l[0] = 0;
               l[1] = j2;
               l[2] = j3;

               let iter = (j3..=n).filter(|x| (j2 == 1) || euclid_u32(*x, n) >= j2);
               'inner: for it in iter.combinations_with_replacement(len-3) {

                   for i in 3..len {
                       l[i] = it[i-3];
                   }

                   // Remove some cases made redundant by complex conjugation.
                   if (l[len-1] < n) && (l[2] + l[len-1] > n + l[1]) {
                       continue 'inner;
                   }

                   // Skip cases where two roots of unity differ by a factor of -1.
                   for a in 0..len {
                       if l[a] < n {
                           for b in 0..a {
                               if l[a] == l[b] + n2 {
                                   continue 'inner;
                               }
                           }
                       }
                   }

                   // Skip cases where two roots of unity differ by a factor of zeta_3.
                   if n3 != 0 {
                       for a in 0..len {
                           if l[a] < n {
                               for b in 0..a {
                                   if (l[a] == l[b] + n3) || (l[a] == l[b] + 2*n3) {
                                       continue 'inner;
                                   }
                               }
                           }
                       }
                   }

                   // Skip cases where three roots of unity differ by factors of zeta_5.
                   if n5 != 0 {
                       for a in 0..len {
                           if l[a] < n {
                               for b in 0..a {
                                   if (l[a] > l[b]) && ((l[a]-l[b]) % n5 == 0) {
                                       for c in 0..b {
                                           if (l[b] > l[c]) && ((l[b]-l[c]) % n5 == 0) {
                                               continue 'inner;
                                           }
                                       }
                                   }
                               }
                           }
                       }
                   }

                   // Filter for house squared <= 5.01.
                   let ex = CyclotomicIntegerExponents{ exponents: &l, level: n, cos_table: &cos_table_local, sin_table: &sin_table_local };
                   if !ex.compare_house_squared(5.01 as f64) {
                      continue 'inner;
                   }
                   
                   // Skip cases visibly of form (2) of Cassels's theorem.
                   if l[3] == n {
                       if (l[2] == n/2 - l[1]) || (l[2] == n/2 + 2*l[1]) || ((2*l[2]) % n == n/2 + l[1]) {
                           continue 'inner;
                       }
                   }
                   
                   // Skip cases visibly of form (3) of Cassels's theorem.
                   if (n5 != 0) && (l[3] != n) && ((len == 4) || (l[4] == n)) {
                       for (i, i1, i2) in [(1,2,3), (2,1,3), (3,1,2)] {
                           if ((l[i] - l[0]) % n5 == 0) && ((l[i2] - l[i1]) % n5 == 0) && (l[i] - l[0] != l[i2] - l[i1]) && (l[1] - l[0] + l[i2] - l[i1] != n) {
                               continue 'inner;
                           }
                       }
                   }

                   // Skip cases where four roots of unity differ by factors of zeta_7.
                   if n7 != 0 {
                       for a in 0..len {
                           if l[a] < n {
                               for b in 0..a {
                                   if (l[a] > l[b]) && ((l[a]-l[b]) % n7 == 0) {
                                       for c in 0..b {
                                           if (l[b] > l[c]) && ((l[b]-l[c]) % n7 == 0) {
                                               for d in 0..c {
                                                   if (l[c] > l[d]) && ((l[c]-l[d]) % n7 == 0) {
                                                       continue 'inner;
                                                   }
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
               println!("Checked cases with n = {}, j_2 = {}, j_3 = {}", n, j2, j3);
             });
         }

        // Record the level and exponents from all spawned threads
        drop(tx);
        for l in rx {
            println!("{:?}", l);
            write!(f2, "{}; {:?}\n", n, l).expect("output failure");
        }
    }

}

fn main() -> std::io::Result<()> {
   test_cyclotomic_integer_exponents();
   println!("All tests passed!");

   let f1 = File::create("tables.txt")?;
   let f2 = File::create("output.txt")?;
   loop_over_roots(19, 9, &f1, &f2);
   loop_over_roots(2*2*3*7, 8, &f1, &f2);
   loop_over_roots(2*2*3*5, 8, &f1, &f2);
   loop_over_roots(7*13, 7, &f1, &f2);
   loop_over_roots(2*2*3*5*7, 7, &f1, &f2);
   loop_over_roots(31, 6, &f1, &f2);
   loop_over_roots(29, 6, &f1, &f2);
   loop_over_roots(23, 6, &f1, &f2);
   loop_over_roots(2*2*13, 6, &f1, &f2);
   loop_over_roots(3*5*11, 6, &f1, &f2);
   loop_over_roots(3*3*7, 6, &f1, &f2);
   loop_over_roots(3*3*5, 6, &f1, &f2);
   loop_over_roots(3*7*11*13, 5, &f1, &f2);
   loop_over_roots(5*13, 5, &f1, &f2);
   loop_over_roots(2*2*3*5*7*11, 5, &f1, &f2);
   loop_over_roots(5*19, 4, &f1, &f2);
   loop_over_roots(5*17, 4, &f1, &f2);
   loop_over_roots(2*2*3*5*7*11*13, 4, &f1, &f2);
   loop_over_roots(2*2*2*3*3*5*7, 4, &f1, &f2);
   loop_over_roots(2*2*2*2*3*5, 4, &f1, &f2);

   println!("All cases checked!");
   Ok(())

}
