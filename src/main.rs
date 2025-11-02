use std::env;
use std::fs;
fn main() {
let args: Vec<String> = env::args().collect();
let query = &args[1];
let filename = &args[2];
println!("Searching for {}", query);
println!("In file {}", filename);
let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}


//This is a Rust program to replicate the Grep prograp.
//std::env::args is used to get the command line arguments passed to the program.