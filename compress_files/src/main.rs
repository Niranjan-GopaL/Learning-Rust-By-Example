extern crate flate2;

// fn of flate2 we are using
use flate2::Compression;
use flate2::write::GzEncoder;

// inorder to except arguments from user in cli
use std::env::args;

// 
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

}

