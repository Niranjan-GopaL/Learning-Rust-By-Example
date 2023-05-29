use std::error::Error;
// use std::io::Error this is what I used , but turns out there is d
use std::fs::File;

/// main fucntin is special function 
/// there are certain restriction on what it can return 
/// and what it can take in as parameters
fn main() -> Result<(),Box<dyn Error>> {

    let _f = File::open("path/to/file.txt")?;
    Ok(())

}