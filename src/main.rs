mod parser;
use parser::Parser;
use std::env;

fn main() {
    let mut file_name = String::new(); // last argument.
    for filename in env::args() {
        file_name = filename;
    }
    let parser = Parser::init(file_name);

    let mut row_counter = 0;
    for line in parser.read_lines() {
        row_counter += 1;
        if let Ok(row) = line {
            let row = match parser.parse(row) {
                Err(message) => message, // case of commented or empty rows.
                Ok(row) => row,
            };
            if row.is_empty() {
                continue;
            }
            println!("{:?}", row);
            let command = row.split_whitespace();
        } else {
            panic!("Bad row {row_counter}, check if the rows are correct.");
        }
    }
}
