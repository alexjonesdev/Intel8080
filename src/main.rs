//Imports
use std::env;

//Disassemble function
fn disassemble(){
    return;
}

//Program entry point
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
