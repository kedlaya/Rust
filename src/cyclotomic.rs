// use crate::integers;
use gcd::euclid_u32;  // I'm sad this is not in the standard library

// First: standard library
use std::f64::consts::TAU;

pub fn cosine_sine_table(n: u32) -> (Vec<f64>, Vec<f64>) {
    let mut cos_table: Vec<f64> = Vec::new();
    let mut sin_table: Vec<f64> = Vec::new();
    let angle0 = TAU / (n as f64);

    for j in 0..n {
        cos_table.push((angle0 * (j as f64)).cos());
        sin_table.push((angle0 * (j as f64)).sin());
    }
    (cos_table, sin_table)
}

pub struct CyclotomicIntegerExponents<'a> {
    pub exponents: &'a Vec<u32>,
    pub level: u32,
    pub cos_table: &'a Vec<f64>,
    pub sin_table: &'a Vec<f64>
}

impl CyclotomicIntegerExponents<'_> {

    fn conjugates_abs_squared(&self) -> impl Iterator<Item = f64> {
        /// Iterate through the squares of the modules of the conjugates
        /// of self. We use `abs` to stick the SageMath convention.

        // Here is an attempt at creating an iterator. Unfortunately,
        // it's more complicated than simply using `yield` as in Python.

        // Iterate through the conjugates
        (1..self.level)
            // First, get the right Galois group automorphisms:
            .filter(move |k| euclid_u32(*k, self.level) == 1)
            // Second, compute the square of the modulus for this Galois
            // automorphism:
            .map(move |k| {
                let mut cos_sum: f64 = 0.0;
                let mut sin_sum: f64 = 0.0;
                for j in self.exponents {
                    if *j < self.level {
                        let i = ((k*j)%self.level) as usize;
                        cos_sum += self.cos_table[i];
                        sin_sum += self.sin_table[i];
                    }
                }

                // Yield the square of the modulus
                cos_sum.powi(2) + sin_sum.powi(2)
            })
        // From my very limited understanding, the `move` keyword is
        // used to transfer ownership of any variable appearing in the
        // closure definition, to the closure itself. In our case:
        // self.level in the first closure, and &self.exponents and
        // self.level.
    }

    pub fn house_squared(&self) -> f64 {
        /// Return the square of the house of the input.

        // TODO: It would be more idiomatic to check for emptyness of the
        // iterator rather than return 0. The return type would probably be
        // something along the lines of Option(f64). Same for the next
        // method.

        let mut max_abs_squared = 0 as f64;
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

fn float_equality(x: f64, y: f64) -> bool {
    // WARNING: At some point, we probably want that in another file.
    (x - y).abs() < (0.000001 as f64)
}

pub fn test_cyclotomic_integer_exponents() {
    // Tests for CyclotomicIntegerExponents

    // Test 1
    // Randomly taken from SageMath
    let (cos_table, sin_table) = cosine_sine_table(7);
    let l = vec![0, 1, 3, 5];
    let ex1 = CyclotomicIntegerExponents{ exponents: &l,
                                          level: 7,
                                          cos_table: &cos_table,
                                          sin_table: &sin_table
    };
    let sage_res1: f64 = 5.04891733952231;
    assert!(float_equality(ex1.house_squared(), sage_res1));
    assert!(ex1.compare_house_squared(sage_res1+0.000001));
    assert!(!ex1.compare_house_squared(5 as f64));

    // Test 2
    // Taken from table 1 of Kiran's notes
    let (cos_table, sin_table) = cosine_sine_table(31);
    let l = vec![0, 1, 3, 8, 12, 18];
    let ex2 = CyclotomicIntegerExponents{ exponents: &l,
                                          level: 31,
                                          cos_table: &cos_table,
                                          sin_table: &sin_table
    };
    assert!(float_equality(ex2.house_squared(), 5 as f64));
    assert!(ex2.compare_house_squared(5.000001 as f64));

    // Test 3
    // Taken from table 1 of Kiran's notes
    let (cos_table, sin_table) = cosine_sine_table(70);
    let l = vec![0, 1, 11, 42, 51];
    let ex3 = CyclotomicIntegerExponents{ exponents: &l,
                                          level: 70,
                                          cos_table: &cos_table,
                                          sin_table: &sin_table
    };
    assert!(float_equality(ex3.house_squared(), 3 as f64));
    assert!(ex3.compare_house_squared(3.000001 as f64));
    assert!(!ex3.compare_house_squared(2.999999 as f64));

    // Test 4
    // i (imaginary unit)
    let (cos_table, sin_table) = cosine_sine_table(4);
    let l = vec![1];
    let ex4 = CyclotomicIntegerExponents{ exponents: &l,
                                          level: 4,
                                          cos_table: &cos_table,
                                          sin_table: &sin_table
    };
    assert_eq!(ex4.house_squared(), 1 as f64);

    // Test 5
    // 1+i (imaginary unit)
    let (cos_table, sin_table) = cosine_sine_table(4);
    let l = vec![0, 1];
    let ex5 = CyclotomicIntegerExponents{ exponents: &l,
                                          level: 4,
                                          cos_table: &cos_table,
                                          sin_table: &sin_table
    };
    assert_eq!(ex5.house_squared(), 2 as f64);
}

