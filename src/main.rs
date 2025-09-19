mod cyclotomic;
mod cassels;

use cassels::loop_over_roots;

use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let file_tables = File::create("tables.txt")?;
    let file_output = File::create("output.txt")?;
    let inputs = [(19,              9),
                  (2*2*3*7,         8),
                  (2*2*3*5,         8),
                  (7*13,            7),
                  (2*2*3*5*7,       7),
                  (31,              6),
                  (29,              6),
                  (23,              6),
                  (2*2*13,          6),
                  (3*5*11,          6),
                  (3*3*7,           6),
                  (3*3*5,           6),
                  (3*7*11*13,       5),
                  (3*5*7*13,        5),
                  (2*2*3*5*7*11,    5),
                  (5*19,            4),
                  (5*17,            4),
                  (2*2*3*5*7*11*13, 4),
                  (2*2*2*3*3*5*7,   4),
                  (2*2*2*2*3*5,     4)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    println!("All cases checked!");
    Ok(())
}
