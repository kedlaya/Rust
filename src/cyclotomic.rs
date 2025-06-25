// use crate::integers;
use gcd::euclid_u32;  // I'm sad this is not in the standard library

// First: standard library
use std::f64::consts::TAU;


pub struct CyclotomicIntegerExponents {
    exponents: Vec<u32>,
    level: u32,
}


impl CyclotomicIntegerExponents {

    pub fn house_squared(&self) -> f64 {
    /// Return the square of the house of the input.

        let mut max_house_squared: f64 = 0.0;

        // Iterate through the conjugates
        for k in 1..self.level as u32 {
            if euclid_u32(k, self.level) != 1 {
                continue;
            }

            // Compute the house squared
            let mut cos_sum = 0.0;
            let mut sin_sum = 0.0;
            for j in &self.exponents {
                let angle = TAU * ((k * j % self.level) as f64) /
                            (self.level as f64);
                let (sin, cos) = angle.sin_cos();
                cos_sum += cos;
                sin_sum += sin;
            }
            let house_squared = cos_sum.powi(2) + sin_sum.powi(2);

            // Compare the house squared
            if house_squared > max_house_squared {
                max_house_squared = house_squared;
            }
        }
        max_house_squared
    }
}
