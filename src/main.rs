//Imports
use std::env;
use std::fs;

//Returns the op code for a given byte
// fn get_assembly(byte: &u8) -> i32{
//     let length: i32 = 0;
//     match byte {
//         0x00 => println!(""); length = 1,
//         _ => println!("No command found for opcode '{}'", byte)
//     }
//     return length;
// }

//Disassemble function
fn disassemble(bytes: &Vec<u8>) {
    let mut i = 0;
    while i < 15 {//bytes.len() {
        let mut ichange = 1;
        match bytes[i] {
        0x00 => println!("NOP"),
        0x01 => {println!("LXI    B,0x{1:X}{0:X}", bytes[i + 1], bytes[i + 2]); ichange = 3;},
        0xc3 => {println!("JMP    0x{1:X}{0:X}", bytes[i + 1], bytes[i + 2]); ichange = 3},
        _ => println!("No command found for opcode '{:#X?}'", bytes[i])
        }
        i += ichange;
    }
}

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

    disassemble(&contents);

    // for i in 0..contents.len(){
    //     if contents[i] == 0x00 {
    //         print!("True")
    //     }
    //     else {
    //         print!("False")
    //     }
    //     //println!("{:#X?}", contents[i])
    // }
    //println!("{:#X?}", contents);
    //println!("With text:\n{}", contents);
}