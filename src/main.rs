mod cyclotomic;
mod cassels;

use cassels::{loop_over_roots, Output, sort_output_file};

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file_tables = File::create("tables.txt")?;
    let file_output = File::create("output.txt")?;
    let inputs = [(1, 1)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    let output = Output::new(String::from("0; [0]")).unwrap();
    let output2 = Output::new(String::from("0; [1]")).unwrap();

    println!("{}", output < output2);

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
                  (5*13,            5),
                  (2*2*3*5*7*11,    5),
                  (5*19,            4),
                  (5*17,            4),
                  (2*2*3*5*7*11*13, 4),
                  (2*2*2*3*3*5*7,   4),
                  (2*2*2*2*3*5,     4)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    let file_output = File::open("output.txt")?;
    let file_output_sorted = File::create("output.txt.sorted")?;
    sort_output_file(&file_output, &file_output_sorted);

    println!("All cases checked!");
    Ok(())
}
