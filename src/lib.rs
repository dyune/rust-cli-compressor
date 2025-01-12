use clap::Parser;
use std::{
    fs,
    io
};

#[derive(Parser, Debug)]
#[command(name = "CLF", version = "0.1")]
pub struct Args {

    #[arg(
        short,
        long,
        required = true,
        help = "Path to the document to be compressed")
    ]
    path: String,

    #[arg(
        short,
        long,
        required = false,
        help = "Path to the directory where the document will output")
    ]
    destination: String,

}

pub fn parse_args() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input = Args::parse();
    let mut args: Vec<String> = vec!();

    args.push(input.path);
    args.push(input.destination);
    println!("{:?}", args);

    Ok(args)

}

// TODO: Return actual file itself, not strings.
pub fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file;

    if let Ok(path) = fs::canonicalize(path) {
        file = fs::read_to_string(path)?;
        Ok(file)
    } else {
        eprintln!("Could not read file at: {} as it is a directory.", path);
        Err(io::Error::from(io::ErrorKind::NotFound).into())
    }
}


