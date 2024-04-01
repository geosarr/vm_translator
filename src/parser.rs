use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Taken from ?
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Parser<P>
where
    P: AsRef<Path>,
{
    filename: P,
}

impl<P: AsRef<Path>> Parser<P> {
    pub fn init(filename: P) -> Self {
        Self { filename }
    }
    pub fn filename(&self) -> &P {
        &self.filename
    }
    pub fn read_lines(&self)
    where
        P: Clone,
    {
        if let Ok(lines) = read_lines(self.filename.clone()) {
            for line in lines {
                if let Ok(row) = line {
                    println!("{:?}", row);
                } else {
                    panic!("bad row, check if the rows are correct.");
                }
                // counter += 1;
            }
        } else {
            panic!("Error in file, check its content, the file absolute path.")
        }
    }
}
