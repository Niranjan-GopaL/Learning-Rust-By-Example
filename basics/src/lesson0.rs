use std::io;

// #![allow(dead_code)]




pub fn lesson_0() {
    // Accept a value from the user
    println!("Enter a value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim whitespace and convert to the desired type
    let value: u32 = input.trim().parse().expect("Invalid input");

    println!("{}", value);






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






    // use {:?} to print tuples or {:#?} to print it in a pretty way
    let t = (1,2,3,4,5);
    
    println!("{:?}",t);
    println!("{:#?}",t);

    // t.0 t.1 t.2 etc

    add(1, 2);

    
    /* Multi line comments

     */

}   


pub fn add(a:i32,b:i32) -> i32 {
    a + b
}


pub fn _maximum_of_three(a:i32,b:i32,c:i32) -> i32 {
    if a>b
        {
        if a>c
        {
            return a;
        }else{
            return c;
        }
    }else{
        if b>c
        {
            return b;
        }else{
            return c;
        }
    }
}



// No idea why this can't work

// error :- 

// if` may be missing an `else` clause
// `if` expressions without `else` evaluate to `()`
// consider adding an `else` block that evaluates to the expected type

// pub fn _maximum_of_three__2(a:i32,b:i32,c:i32) -> i32 {
//     if a > b && a > c {
//         return a
//     }
//     if b > a && b > c {
//         return b
//     }
//     if c > a && c > b {
//         return c
//     }
// }