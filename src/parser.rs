use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Lines = io::Lines<io::BufReader<File>>;

// Taken from ?
pub fn read_lines<P>(filename: P) -> io::Result<Lines>
where
    P: AsRef<Path>,
{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
pub enum Command {
    ARITHMETIC,
    CALL,
    FUNCTION,
    GOTO,
    IF,
    LABEL,
    POP,
    PUSH,
    RETURN,
}

#[derive(Debug, Clone)]
pub struct Args {
    command: Option<Command>,
    one: Option<String>,
    two: Option<String>,
    three: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Self {
            command: None,
            one: None,
            two: None,
            three: None,
        }
    }
    pub fn mutate_args(&mut self, arg: String, number: usize) {
        assert!(1 <= number && number <= 3);
        if number == 1 {
            self.one = Some(arg.clone());
            self.mutate_command(arg);
        } else if number == 2 {
            self.two = Some(arg);
        } else {
            self.three = Some(arg);
        }
    }
    pub fn mutate_command(&mut self, arg: String) {
        self.command = Some(
            if arg.starts_with("add")
                || arg.starts_with("and")
                || arg.starts_with("or")
                || arg.starts_with("neg")
                || arg.starts_with("not")
                || arg.starts_with("sub")
                || arg.starts_with("lt")
                || arg.starts_with("gt")
                || arg.starts_with("eq")
            {
                Command::ARITHMETIC
            } else if arg.starts_with("pop") {
                Command::POP
            } else {
                Command::PUSH
            },
        );
    }
    pub fn is_empty(&self) -> bool {
        return self.one.is_none() && self.two.is_none() && self.three.is_none();
        // return self.command.is_none();
    }
}

#[derive(Debug)]
pub struct Parser<P>
where
    P: AsRef<Path>,
{
    filename: P,
    parsed_args: Args,
}

impl<P: AsRef<Path>> Parser<P> {
    pub fn init(filename: P) -> Self {
        Self {
            filename,
            parsed_args: Args::new(),
        }
    }

    pub fn filename(&self) -> &P {
        &self.filename
    }

    pub fn parsed_args(&self) -> &Args {
        &self.parsed_args
    }

    pub fn parse(&mut self, row: String) -> Result<Args, String> {
        let out_row = row.trim();
        if out_row.starts_with("//") || out_row.is_empty() {
            Err("".to_string())
        } else {
            let command_args = row.split_whitespace();
            let mut args = Args::new();
            for (number, arg) in command_args.enumerate() {
                args.mutate_args(arg.to_string(), number + 1);
            }
            self.parsed_args = args.clone();
            Ok(args)
        }
    }

    pub fn read_lines(&self) -> Lines
    where
        P: Clone,
    {
        if let Ok(lines) = read_lines(self.filename()) {
            lines
        } else {
            panic!("Error in file, check its content, the file absolute path.")
        }
    }
}
