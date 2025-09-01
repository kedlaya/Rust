mod cyclotomic;
mod cassels;

use cassels::loop_over_roots;

use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let file_tables = File::create("tables.txt")?;
    let file_output = File::create("output.txt")?;
    let inputs = [(1, 1)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    let inputs = [(19,              9),
                  (2*2*3*5,         8),
                  (2*2*2*2*3*5,     4)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    println!("All cases checked!");
    Ok(())
}
