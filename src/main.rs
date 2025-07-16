mod cyclotomic;

use cyclotomic::{CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use itertools::Itertools;

use gcd::euclid_u32;

fn loop_over_roots(n: u32, len: usize, mut f: &File) {
    let n2 = if n%2 == 0 {n / 2} else {0};
    let n3 = if (n2 != 0) && (n%3 == 0) {n / 3} else {0};
    let n5 = if (n2 != 0) && (n%5 == 0) {n / 5} else {0};
    let n7 = if (n2 != 0) && (n%7 == 0) {n / 7} else {0};

    for j2 in 1..n {
       // Require that j_2 divides n.
       if n % j2 != 0 {
           continue;
       }
       // Loop over tuples [j_3, ..., j_*] with 0 <= j_3 <= ... <= j_* <= n,
       // also requiring that gcd(j_i, n) >= j_2.
       // Note: we allow n as an exponent as a proxy for a zero summand.
       let (tx, rx) = mpsc::channel();
       for j3 in (0..=n).filter(|x| euclid_u32(*x, n) >= j2) {
           let tx_clone = tx.clone();
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
                   if n2 != 0 {
                       for a in 0..len {
                           if l[a] < n {
                               for b in 0..a {
                                   if l[a] == l[b] + n2 {
                                       continue 'inner;
                                   }
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

                   // Filter for house squared <= 5.1
                   let ex = CyclotomicIntegerExponents{ exponents: l.clone(), level: n };
                   if !ex.compare_house_squared(5.1 as f64) {
                      continue 'inner;
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
            write!(f, "{}; {:?}\n", n, l).expect("output failure");
        }
    }

}

fn main() -> std::io::Result<()> {
   test_cyclotomic_integer_exponents();
   println!("All tests passed!");

   let f = File::create("output.txt")?;
   loop_over_roots(2*31, 6, &f);
   loop_over_roots(2*23, 6, &f);
   loop_over_roots(2*19, 9, &f);
   loop_over_roots(2*3*13, 7, &f);
   loop_over_roots(2*5*13, 5, &f);
   loop_over_roots(2*2*3*5*7*11*13, 4, &f);
   loop_over_roots(2*3*7*11*13, 5, &f);
   loop_over_roots(2*3*5*11, 6, &f);
   loop_over_roots(2*2*3*5*7*11, 5, &f);
   loop_over_roots(2*2*3*5*7, 7, &f);

   println!("All cases checked!");
   Ok(())

}
