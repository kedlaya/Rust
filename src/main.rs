mod cyclotomic;
mod cassels;

use cassels::loop_over_roots;

use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let file_tables = File::create("tables.txt")?;
    let file_output = File::create("output.txt")?;
    let inputs = [(2*2*3*5*7,       7),  // Proposition 4.3
                  (31,              6),  // Remark 8.3
                  (3*5*7*13,        5),  // Section 8.3.1
                  (2*2*3*5*7*11,    5),  // Sections 4.2.1, 8.2.1
                  (5*19,            4),  // Section 4.2.4
                  (5*17,            4),  // Section 4.2.4
                  (2*2*3*5*7*11*13, 4),  // Section 4.2.2
                  (2*2*2*3*3*5*7,   4)]; // Proposition 4.1
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    println!("All cases checked!");
    Ok(())
}
