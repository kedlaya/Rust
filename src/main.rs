mod cyclotomic;

use cyclotomic::{CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
   test_cyclotomic_integer_exponents();
   let mut f = File::create("output.txt")?;
   let n = 420;
   let n2 = n / 2;
   let n3 = n / 3;
   let len = 7;

   for d in 1..n {
      if n % d != 0 {
          continue;
      }
      // Note: we allow n as an exponent as a proxy for a zero summand.
      let r = (n / d) + 1;
      // Loop over tuples [j_1, ..., j_5] with 0 <= j1 <= ... <= j5 <= n/d
      for j1 in 0..r {
         println!("Checking cases with d = {}, j_0 = {}", d, j1);
         for j2 in j1..r {
            for j3 in j2..r {
               for j4 in j3..r {
                  'inner: for j5 in j4..r {
                     let l = vec![0, d, d*j1, d*j2, d*j3, d*j4, d*j5];

                     // Remove some cases made redundant by complex conjugation.
                     if (l[len-1] < n) && (l[2]-l[1] > n - l[len-1]) {
                         continue;
                     }

                     // Skip cases where two roots of unity sum to zero
                     for a in 0..len {
                        for b in 0..a {
                           if (l[a] < n) && (l[a] == l[b] + n2) {
                              break 'inner;
                           }
                        }
                     }

                     // Skip cases where three roots of unity sum to zero
                     for a in 0..len {
                        for b in 0..a {
                           for c in 0..b {
                              if (l[a] < n) && (l[a] == l[b] + n3) && (l[b] == l[c] + n3) {
                                 break 'inner;
                              }
                           }
                        }
                     }

                     // Filter for house squared <= 5.1
                     let ex = CyclotomicIntegerExponents{ exponents: l, level: n };
                     if ex.compare_house_squared(5.1 as f64) {
                        // Record the exponents (omitting the initial 0)
                        write!(f, "[{}, {}, {}, {}, {}, {}]\n", l[1], l[2], l[3], l[4], l[5], l[6]).expect("output failure");
                     }
                  }
               }
            }
         }
      }
   }
   println!("All cases checked!");
   Ok(())
}
