// use crate::integers;
use gcd::euclid_u32;  // I'm sad this is not in the standard library

// First: standard library
use std::f64::consts::TAU;
// Second: our own library
// use integers::{euler_phi, invertible_mod};

// pub struct CyclotomicInteger {
//     vec: Vec<i32>,
// }

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

// impl CyclotomicInteger {
// 
//     //////////////////
//     // CONSTRUCTORS //
//     //////////////////
// 
//     pub fn from_vec(vec: Vec<i32>) -> Self {
//         
//         Self { vec }
// 
//     }
// 
//     pub fn from_hashmap(hashmap: HashMap<usize, i32>, level: usize) -> Self {
// 
//         // Note that we use the hashmap and not a reference. This seems
//         // more idiomatic: our hashmaps are only used to create a
//         // CyclotomicObject. This clearly indicates that the function
//         // `from_hashmap` takes ownership of the input hashmap. This is
//         // based on the implicit assumption that the hashmap will not be
//         // used after.
// 
//         if level == 0 {
//             panic!("no 0th-roots of unity");
//         }
// 
//         let mut vec = vec![0 as i32; level];
// 
//         for (key, val) in hashmap.into_iter() {
//             if key >= level {
//                 panic!("hashmap keys should be < than the level")
//             } else {
//                 vec[key] = val;
//             }
//         }
// 
//         Self { vec }
// 
//     }
// 
// 
//     ///////////
//     // UTILS //
//     ///////////
// 
//     pub fn level(&self) -> usize {
//         self.vec.len()
//     }
// 
//     pub fn support(&self) -> HashMap<usize, i32> {
//     
//         let mut support = HashMap::new();
// 
//         for i in 0..self.level() {
//             if self[i] != 0 as i32 {
//                 support.insert(i, self.vec[i]);
//             }
//         }
//     
//         support
// 
//     }
// 
// 
//     //////////////////
//     // COMPUTATIONS //
//     //////////////////
// 
//     pub fn conjugates(&self) -> Vec<CyclotomicInteger> {
// 
//         // First, create euler_phi(n) vectors of length `level`:
//         let level = self.level();
//         let euler = euler_phi(level as u32);
//         let mut conjugates_vec: Vec<Vec<i32>> = Vec::new();
//         // Now, store the Galois group
//         let galois = invertible_mod(level as u32);
// 
//         // Generate all conjugates, without repetition
//         for k in galois {
//             // Generate the conjugate for the k-th Galois automorphism:
//             let mut conjugate_vec = vec![0 as i32; level];
//             for i in 0..level {
//                 // TODO: Implement `iter` for our struct
//                 let index = (i * (k as usize)) % level;
//                 conjugate_vec[index] = self[i];
//             }
//             // Check if the conjugate already exists:
//             // (We acknowledge that an Hash-thing would be more
//             // efficient. However, we would loose the order.
//             // We're not sure if we might need it, but let's keep
//             // it simple for now.)
//             if !conjugates_vec.contains(&conjugate_vec) {
//                 conjugates_vec.push(conjugate_vec)
//             }
//         }
// 
//         // Cast to CyclotomicInteger
//         let mut conjugates: Vec<CyclotomicInteger> = Vec::new();
//         for conjugate_vec in conjugates_vec {
//             let conjugate = CyclotomicInteger::from_vec(conjugate_vec);
//             conjugates.push(conjugate);
//         }
//         
//         conjugates
//     }
//     
// }


// ////////////
// // TRAITS //
// ////////////
// 
// // I just used an LLM for this one... One can also simply add
// // #[derive(Debug)] before the struct declaration, but this yields a
// // cleaner result. Although, I understand the code less. Also, note that
// // we implement the `Debug` trait, so the formatter is "{:?}" and not
// // "{}".
// 
// impl fmt::Debug for CyclotomicInteger {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.vec.fmt(f) 
//     }
// }
// 
// // The next one is to define the [ ] access operator. See the doc here:
// //     https://doc.rust-lang.org/std/ops/trait.Index.html
// 
// impl Index<usize> for CyclotomicInteger {
//     type Output = i32;
//     fn index(&self, i: usize) -> &Self::Output {
//         &self.vec[i]
//     }
// }
