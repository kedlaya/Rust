mod integers;
mod cyclotomic;

use std::collections::HashMap;

use integers::euler_phi;
use cyclotomic::CyclotomicIntegerExponents;


fn main() {
    println!("Hello, world!");

    println!("{}", euler_phi(420));

    // let mut a_hashmap = HashMap::new();
    // a_hashmap.insert(0 as usize, 263    as i32);
    // a_hashmap.insert(3 as usize, -12748 as i32);
    // let a = CyclotomicInteger::from_hashmap(a_hashmap, 10 as usize);

    // println!("{:?}", a);
    // println!("{:?}", a.support());

    // // Let's do some testing!

    // let i_Q4 = CyclotomicInteger::from_vec(vec![0 as i32,
    //                                             1 as i32,
    //                                             0 as i32,
    //                                             0 as i32]);

    // let i_Q8 = CyclotomicInteger::from_vec(vec![0 as i32,
    //                                             0 as i32,
    //                                             1 as i32,
    //                                             0 as i32,
    //                                             0 as i32,
    //                                             0 as i32,
    //                                             0 as i32,
    //                                             0 as i32]);

    // println!("{:?}", i_Q4.conjugates());
    // println!("{:?}", i_Q8.conjugates());

}
