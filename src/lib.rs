use clap::Parser;
use std::io::{self, BufRead};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// The input file to read.
    #[arg(short, long, default_value_t=String::from("-"))]
    pub infile: String,
    /// Compute part two of the puzzle.
    #[arg(short, long, default_value_t=false)]
    pub part_two: bool,
}


pub struct Support {
    pub args : Args,
    pub lines: Vec<io::Result<String>>,
}

impl Support {
    pub fn new() -> io::Result<Self> {
        let args = Args::parse();
        let bufread: Box<dyn io::BufRead> = {
            if args.infile == "-" {
                Box::new(io::stdin().lock())
            }
            else {
                let f = fs::File::open(&args.infile)?;
                let f = io::BufReader::new(f);
                Box::new(f)
            }
        };

        let lines: Vec<io::Result<String>> = bufread.lines().collect();
        Ok(Support {
            args,
            lines
        })
    }
}
