use gcd::euclid_u32;  // I'm sad this is not in the standard library

// TODO: Unit tests

pub fn euler_phi(n: u32) -> u32 {
    /// Euler totient function.
    
    let mut res: u32 = 0;

    for i in 0..n {
        if euclid_u32(i as u32, n) == 1 {
            res += 1
        }
    }

    res
}

pub fn invertible_mod(n: u32) -> Vec<u32> {
    /// Return the list of invertible elements (represented as positive
    /// reduced integers) modulo the input.
    
    let mut res: Vec<u32> = Vec::new();

    for i in 0..n {
        if euclid_u32(i as u32, n) == 1 {
            res.push(i as u32)
        }
    }

    res

}
