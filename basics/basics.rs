use std::io;

fn main() {
    // Accept a value from the user
    println!("Enter a value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim whitespace and convert to the desired type
    let value: i32 = input.trim().parse().expect("Invalid input");

    // This won't compile cuz compiler things enigma could be 
    //uninitialised since it is in an if statement
    // let enigma1:u8;
    // if true{
    //     enigma1 = 2;
    // }
    // println!("{}",enigma1);
    
    
    // But this would work
    let enigma2:u8;
    if true{
        enigma2 = 2;
    }else{
        enigma2 = 7;
    }
    println!("{}",enigma2);
}
