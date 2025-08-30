use gcd::euclid_u32;  // I'm sad this is not in the standard library

// First: standard library
use std::f64::consts::TAU;

pub fn sin_cos_table(n: u32) -> Vec<(f64, f64)> {
    let angle0 = TAU / (n as f64);
    // Iterate over j
    (0..n)
        // compute sin and cosine of 2*pi*j/n
        .map(|j| (angle0 * (j as f64)).sin_cos())
        // collect and return a vector
        .collect::<Vec<(f64, f64)>>()
}

pub struct CyclotomicIntegerExponents<'a> {
    pub exponents: &'a Vec<u32>,
    pub level: u32,
    pub sin_cos_table: &'a Vec<(f64, f64)>,
}

impl CyclotomicIntegerExponents<'_> {

    fn conjugates_abs_squared(&self) -> impl Iterator<Item = f64> {
        /// Iterate through the squares of the modules of the conjugates
        /// of self. We use `abs` to stick the SageMath convention.

        // Iterate through the conjugates
        (1..self.level)
            // First, get the right Galois group automorphisms:
            .filter(|k| euclid_u32(*k, self.level) == 1)
            // Second, compute the square of the modulus for this Galois
            // automorphism:
            .map(|k| {
                let mut sin_sum: f64 = 0.0;
                let mut cos_sum: f64 = 0.0;
                for j in self.exponents {
                    let i = (k*j % self.level) as usize;
                    // If only we could sum tuples directly...
                    let (sin, cos) = self.sin_cos_table[i];
                    sin_sum += sin;
                    cos_sum += cos;
                }

                // Yield the square of the modulus
                sin_sum.powi(2) + cos_sum.powi(2)
            })
    }

    pub fn house_squared(&self) -> f64 {
        /// Return the square of the house of the input.

        // TODO: It would be more idiomatic to check for emptyness of the
        // iterator rather than return 0. The return type would probably be
        // something along the lines of Option(f64). Same for the next
        // method.

        let mut max_abs_squared = 0_f64;
        for abs_squared in self.conjugates_abs_squared() {
            if abs_squared > max_abs_squared {
                max_abs_squared = abs_squared;
            }
        }
        max_abs_squared
    }

    pub fn compare_house_squared(&self, cutoff: f64) -> bool {
        /// Check whether square of the house of the input is bounded
        /// above by the cutoff. This is more efficient than computing
        /// the house first.

        !self.conjugates_abs_squared().any(|x| x >= cutoff)
    }
}

// For idiomatic doctesting, see
// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
// Basically, add tests in the module below, prefix with #[test],
// and run `cargo test`.

#[cfg(test)]
mod tests {

    use super::*;

    fn float_equality(x: f64, y: f64) -> bool {
        (x - y).abs() < (0.000001_f64)
    }

    #[test]
    fn test_castle() {
        // Tests for CyclotomicIntegerExponents

        // This is necessary as otherwise there is a conflict between the variable sin_cos_table,
        // and the function. This is called shadowing, see `rustc --explain E0618`.
        let sin_cos_table_fn = sin_cos_table;

        // Test 1
        // Randomly taken from SageMath
        let sin_cos_table = sin_cos_table_fn(7);
        let l = vec![0, 1, 3, 5];
        let ex1 = CyclotomicIntegerExponents{ exponents: &l,
                                              level: 7,
                                              sin_cos_table: &sin_cos_table
        };
        let sage_res1: f64 = 5.04891733952231;
        assert!(float_equality(ex1.house_squared(), sage_res1));
        assert!(ex1.compare_house_squared(sage_res1+0.000001));
        assert!(!ex1.compare_house_squared(5 as f64));

        // Test 2
        // Taken from table 1 of Kiran's notes
        let sin_cos_table = sin_cos_table_fn(31);
        let l = vec![0, 1, 3, 8, 12, 18];
        let ex2 = CyclotomicIntegerExponents{ exponents: &l,
                                              level: 31,
                                              sin_cos_table: &sin_cos_table
        };
        assert!(float_equality(ex2.house_squared(), 5 as f64));
        assert!(ex2.compare_house_squared(5.000001 as f64));

        // Test 3
        // Taken from table 1 of Kiran's notes
        let sin_cos_table = sin_cos_table_fn(70);
        let l = vec![0, 1, 11, 42, 51];
        let ex3 = CyclotomicIntegerExponents{ exponents: &l,
                                              level: 70,
                                              sin_cos_table: &sin_cos_table
        };
        assert!(float_equality(ex3.house_squared(), 3 as f64));
        assert!(ex3.compare_house_squared(3.000001 as f64));
        assert!(!ex3.compare_house_squared(2.999999 as f64));

        // Test 4
        // i (imaginary unit)
        let sin_cos_table = sin_cos_table_fn(4);
        let l = vec![1];
        let ex4 = CyclotomicIntegerExponents{ exponents: &l,
                                              level: 4,
                                              sin_cos_table: &sin_cos_table
        };
        assert_eq!(ex4.house_squared(), 1 as f64);

        // Test 5
        // 1+i (imaginary unit)
        let sin_cos_table = sin_cos_table_fn(4);
        let l = vec![0, 1];
        let ex5 = CyclotomicIntegerExponents{ exponents: &l,
                                              level: 4,
                                              sin_cos_table: &sin_cos_table
        };
        assert_eq!(ex5.house_squared(), 2 as f64);
    }
}
