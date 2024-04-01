mod parser;
use parser::Parser;
fn main() {
    let parser = Parser::init("src/commands.vm");
    println!("{:?}", parser.filename());
    parser.read_lines();
}
