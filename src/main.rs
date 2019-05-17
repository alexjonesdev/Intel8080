//Imports
use std::env;
use std::fs;
mod disassembler;

//Parse command line arguments
fn parse_arguments(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("Not enough arguments!");
    }

    let filename = &args[1];

    return filename
}

//Program entry point
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_arguments(&args);

    println!("In file {}", filename);

    //let contents = fs::read_to_string(filename)
    let contents = fs::read(filename)
        .expect("Something went wrong reading the file");

    disassembler::disassemble(&contents);
}