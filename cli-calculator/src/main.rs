// io library is used to read and write 

use std::io;
use rand::Rng;
use std::cmp::Ordering;
// use colored; //for some reason this is not working

fn main() {
    println!("<--------------Welcome to Binary Search Game ----------------------->");
    // Rust standard library (the std we mentioned above) does not have a built in a 
    // function to give us a random number. So go to cargo.toml =>
    // Add the dependencies and version number.
    
    // then cargo build (to know if the library was downloaded)
    
    // we'll get rand ; AND ALL THE LIBRARIES THAT RAND DEPENDS on
    
    let secret_number = 
    rand::thread_rng()
    // generates random number between 0 and 10 (both inclusive)
    .gen_range(0, 101);


// GIVE OUT A SHORT DESCRIPTION OF GAME !!!!
    println!("\n\n Enter guess");

// println!("{}",secret_number);

    loop 
    {
        println!("\n\n Enter guess");
        // var for acepting an input from user
        let mut guess = String::new();    
        
    // Constructs a new handle to the standard input of the current process.
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line!!!!!!!!!!!!!!!!!!!!!!!!")
    ;

    // We need to convert guess to a number so that we can compare later
    //Clearly parse fn returns an enum := either Ok or Err
    let guess: i32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("\nYou entered : {}",guess);


    // Now we need to compare the number we guessed with secret_number 
    // Orderting is an enum that is a result of two things being compared
    // Try going and see what is Ordering, what does it return etc

    // Instead of if elif
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small, Try guessing again !"),
        Ordering::Greater => println!("Too Big, Try guessing again !"), 
        // We want to stop when the user wins. SO we can even do this :--|| instead of this :--|
        // Ordering::Equal => println!("{} = {} YOU WIN ",guess,secret_number),
        Ordering::Equal => {
            println!("{} = {} YOU WIN ",guess,secret_number);
            break;
        }
    }
    



    
}



println!("<---------------------------------------------------------------->");
}
