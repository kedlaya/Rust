mod cyclotomic;

use cyclotomic::{CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use itertools::Itertools;

fn loop_over_roots(n: u32, len: usize, mut f: &File) -> std::io::Result<()> {
    let n2 = if n%2 == 0 {n / 2} else {0};
    let n3 = if (n2 != 0) && (n%3 == 0) {n / 3} else {0};
    let n5 = if (n2 != 0) && (n%5 == 0) {n / 5} else {0};

    for d in 1..n {
       if n % d != 0 {
           continue;
       }
       // Loop over tuples [j_1, ..., j_*] with 0 <= j_1 <= ... <= j_* <= n/d.
       // Note: we allow n as an exponent as a proxy for a zero summand.
       let (tx, rx) = mpsc::channel();
       for j1 in (0..=n).step_by(d as usize) {
           let tx1 = tx.clone();
           thread::spawn(move || {
               let mut l: Vec<u32> = vec![0; len];
               l[0] = 0;
               l[1] = d;
               l[2] = j1;

              'inner: for it in ((j1..=n).step_by(d as usize)).combinations_with_replacement(len-3) {

                   for i in 3..len {
                       l[i] = it[i-3];
                   }

                   // Remove some cases made redundant by complex conjugation.
                   if (l[len-1] < n) && ((l[2] + l[len-1] > n + l[1]) || ((l[2] == 0) && (l[3] > 1))) {
                       continue 'inner;
                   }

                   // Skip cases where two roots of unity differ by a factor of -1
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

                   // Skip cases where two roots of unity differ by a factor of zeta_3
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

                   // Skip cases where three roots of unity differ by factors of zeta_5
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
                   if ex.compare_house_squared(5.1 as f64) {
                      tx1.send(l.clone()).unwrap();
                   }
               }
               println!("Checked cases with n = {}, d = {}, j_1 = {}", n, d, j1);
             });
         }

        // Record the level and exponents from all spawned threads
        drop(tx);
        for l in rx {
            println!("{:?}", l);
            write!(f, "{}, {:?}\n", n, l).expect("output failure");
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
   test_cyclotomic_integer_exponents();
   println!("All tests passed!");

   let f = File::create("output.txt")?;
   let _ = loop_over_roots(11, 5, &f);
   let _ = loop_over_roots(420, 7, &f);

   println!("All cases checked!");
   Ok(())
}
