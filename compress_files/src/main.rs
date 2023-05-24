// THE BEST WAY TO KNOW MORE : JUST FUCKING HOVER WHAT YOU WANT 
// THE DOCUMENTAITON IS INSANLY GOOD , VIEW IN VS CDOE ITSELF

extern crate flate2;

// fn of flate2 we are using
use flate2::Compression;
use flate2::write::GzEncoder;
 


// inorder to except arguments from user in cli
use std::env::args;



// when you convert a file to a compressed version 
// (essentially we are copying contents of that file)
use std::fs::File;
// first we read that file
use std::io::BufReader;
// then we copy the contents of that file
use std::io::copy;
// in order to output "how much time the process took" (BENCHMARKING lol!)
use std::time::Instant;


fn main() {

    // if the usage is not correct we'll tell them how to use it
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        eprintln!("source :-------->  file to compress");
        eprintln!(" target:-------->  how should it be named after compression");
        return;
    }

    // 2nd word is the file. `input` is the file that we will be reading
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    // creating an output file
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // turn on the inlay hint (ctrl+alt here) 
    // see that GzEncoder::new(w , level ) you are writing to ouput
    let mut encoder = GzEncoder::new(output, Compression::default());

    // starting the timer -----------------------------------------------------------------------------------------------------
    let start = Instant::now();
    

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    // info on compression process
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len() // SIZE OF FILE IN BYTES
    );
    println!("Target len: {:?}", output.metadata().unwrap().len()); // SIZE OF FILE IN BYTES
    
    // stop timer -----------------------------------------------------------------------------------------------------
    println!("Elapsed: {:?}", start.elapsed());
}

