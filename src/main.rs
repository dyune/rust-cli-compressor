use cli_compressor::*;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(_) => panic!("Invalid arguments"),
    };

    let file = read_file(&args[0]);
    let output = match file {
        Ok(output) => output,
        Err(error) => panic!("Could not read file: {}", error),
    };

    println!("{}", output);

}
