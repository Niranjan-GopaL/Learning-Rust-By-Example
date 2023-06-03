//! minigrep impl
use std::{env,fs};



fn main() {
    let args:Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filepath = &args[2];
    
    println!("Searching for {}",query);
    println!("in file {}",filepath);

    let contents = fs::read_to_dstring(filepath).expect("Something went wrong reading a file ...");

    println!("{}",contents);

}
