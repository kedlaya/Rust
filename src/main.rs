mod cyclotomic;

use cyclotomic::{CyclotomicIntegerExponents, test_cyclotomic_integer_exponents};


fn main() {
   test_cyclotomic_integer_exponents();
   for i1 in 2..420 {
      println!("{}", i1);
      for i2 in i1..420 {
         for i3 in i2..420 {
            'inner: for i4 in i3..420 {
               let l = vec![0,1,i1,i2,i3,i4];
               for a in 0..6 {
                  for b in 0..a {
                     if l[a] - l[b] == 210 {
                        break 'inner;
                     }
                  }
               }
               for a in 0..6 {
                  for b in 0..a {
                     for c in 0..b {
                        if (l[a] - l[b] == 140) & (l[b] - l[c] == 140) {
                           break 'inner;
                        }
                     }
                  }
               }
               let ex = CyclotomicIntegerExponents{ exponents: vec![0, 1, i1, i2, i3, i4],
                                          level: 420 };
               if ex.compare_house_squared(5.1 as f64) {
                   println!("{} {} {} {}", i1, i2, i3, i4);
               }
            }
         }
      }
   }
}
