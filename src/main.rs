mod code_writer;
mod parser;
use code_writer::CodeWriter;
use parser::{Args, Parser};
use std::env;

fn io_file_names(args: std::env::Args) -> (String, String) {
    let mut input_file = String::new();
    let mut output_file = String::new();
    // println!("{:?}", args);
    for arg in args {
        if arg.ends_with("vm") {
            input_file = arg;
        } else if arg.ends_with("asm") || arg.ends_with("hack") {
            output_file = arg;
        }
    }
    (input_file, output_file)
}
fn main() {
    let (input_file, output_file) = io_file_names(env::args());
    let mut parser = Parser::init(input_file);
    let code_writer = CodeWriter::init(output_file);

    let mut row_counter: usize = 0;
    for line in parser.read_lines() {
        row_counter += 1;
        if let Ok(row) = line {
            if row.trim().is_empty() {
                continue;
            }
            let parsed_row = parser.parse(row);
            println!("{:?}", parsed_row);
            println!("{:?}", CodeWriter::<&str>::to_assembly(parsed_row));
        } else {
            panic!("Bad row {row_counter}, check if the rows are correct.");
        }
    }
}
