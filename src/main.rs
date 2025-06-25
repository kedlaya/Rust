mod cyclotomic;

use cyclotomic::{CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
   test_cyclotomic_integer_exponents();
   let mut f = File::create("output.txt")?;
   let n = 420;

   for d in 1..n {
      if n % d != 0 {
          continue;
      }
      // Note: we allow n as an exponent as a proxy for a zero summand.
      let r = (n / d) + 1;
      for j1 in 0..r {
         println!("Checking cases with d = {}, j_0 = {}", d, j1);
         for j2 in j1..r {
            for j3 in j2..r {
               'inner: for j4 in j3..r {
                  let l = vec![0, d, d*j1, d*j2, d*j3, d*j4];

                  // Skip cases where two roots of unity sum to zero
                  for a in 0..6 {
                     for b in 0..a {
                        if (l[a] < n) && (l[a] == l[b] + n/2) {
                           break 'inner;
                        }
                     }
                  }

                  // Skip cases where three roots of unity sum to zero
                  for a in 0..6 {
                     for b in 0..a {
                        for c in 0..b {
                           if (l[a] < n) && (l[a] == l[b] + n/3) && (l[b] == l[c] + n/3) {
                              break 'inner;
                           }
                        }
                     }
                  }

                  // Filter for house squared <= 5.1
                  let ex = CyclotomicIntegerExponents{ exponents: l, level: n };
                  if ex.compare_house_squared(5.1 as f64) {
                     // Record the exponents (omitting the initial 0)
                     write!(f, "[{}, {}, {}, {}, {}]\n", d, d*j1, d*j2, d*j3,
 d*j4).expect("output failure");
                  }
               }
            }
         }
      }
   }
   println!("All cases checked!");
   Ok(())
}
