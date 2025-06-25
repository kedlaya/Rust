// use crate::integers;
use gcd::euclid_u32;  // I'm sad this is not in the standard library

// First: standard library
use std::f64::consts::TAU;


pub struct CyclotomicIntegerExponents {
    pub exponents: Vec<u32>,
    pub level: u32,
}

impl CyclotomicIntegerExponents {

    fn conjugates_abs_squared(&self) -> impl Iterator<Item = f64> {

        let angle0 = TAU / (self.level as f64);

        // Iterate through the conjugates
        (1..self.level)
            // Get the right Galois group automorphisms
            .filter(move |&k| euclid_u32(k, self.level) == 1)
            .map(move |k| {
                // Compute the house squared
                let mut cos_sum = 0.0;
                let mut sin_sum = 0.0;
                for j in &self.exponents {
                    if *j < self.level {
                        let angle = angle0 * ((k * j) as f64);
                        let (sin, cos) = angle.sin_cos();
                        cos_sum += cos;
                        sin_sum += sin;
                    }
                }
                // Yield the house squared
                cos_sum.powi(2) + sin_sum.powi(2)
            })
    }

    pub fn house_squared(&self) -> f64 {
    // Return the square of the house of the input.

        let mut max = 0 as f64;
        for abs_squared in self.conjugates_abs_squared() {
            if abs_squared > max {
                max = abs_squared;
            }
        }
        max
    }

    pub fn compare_house_squared(&self, cutoff: f64) -> bool {
    // Check whether square of the house of the input is bounded above
    // by the cutoff. This is more efficient than computing the
    // house first.

        for abs_squared in self.conjugates_abs_squared() {
            if abs_squared >= cutoff {
                return false;
            }
        }
        true
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
    let ex1 = CyclotomicIntegerExponents{ exponents: vec![0, 1, 3, 5],
                                          level: 7
    };
    let sage_res1: f64 = 5.04891733952231;
    assert!(float_equality(ex1.house_squared(), sage_res1));
    assert!(ex1.compare_house_squared(sage_res1+0.000001));
    assert!(!ex1.compare_house_squared(5 as f64));

    // Test 2
    // Taken from table 1 of Kiran's notes
    let ex2 = CyclotomicIntegerExponents{ exponents: vec![0, 1, 3, 8, 12, 18],
                                          level: 31
    };
    assert!(float_equality(ex2.house_squared(), 5 as f64));
    assert!(ex2.compare_house_squared(5.000001 as f64));

    // Test 3
    // Taken from table 1 of Kiran's notes
    let ex3 = CyclotomicIntegerExponents{ exponents: vec![0, 1, 11, 42, 51],
                                          level: 70
    };
    assert!(float_equality(ex3.house_squared(), 3 as f64));
    assert!(ex3.compare_house_squared(3.000001 as f64));
    assert!(!ex3.compare_house_squared(2.999999 as f64));

    // Test 4
    // i (imaginary unit)
    let ex4 = CyclotomicIntegerExponents{ exponents: vec![1],
                                          level: 4
    };
    assert_eq!(ex4.house_squared(), 1 as f64);

    // Test 4
    // i (imaginary unit)
    let ex5 = CyclotomicIntegerExponents{ exponents: vec![0, 1],
                                          level: 4
    };
    assert_eq!(ex5.house_squared(), 2 as f64);
}

