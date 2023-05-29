//! In the guessing game, the way we validated the input `guess` 
//! is very problematic. there is actually better solution that is more "scalable"

//! IT IS REALLY IMPORTANT THAT YOU HAVE "CUSTOM VALIDATION" 
//! that is SEPERATE from the main logic

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// <------------------------------------ACTUAL WAYYY------------------------------------------------->
pub struct  Guess {
    value:i32,
}

impl Guess {
    pub fn new(value:i32) -> Self {
        if value  <1  || value >100 {panic!("value must be between 1 and 100")} ;
        Self { value }
    }
}

impl Guess {
    pub fn value(&self) -> i32{
        self.value
    }
}
// <------------------------------------------------------------------------------------->



// <------------------------------------------------------------------------------------->
fn main() {
    
    let _guess = 1231; // this is from use
    
    // validating them this way is not good practise
    if _guess > 0 || _guess < 100 { println!("value must be between 1 and 100") }
    
    // THE ENTIRE GUESS VALIDATION IS OUTSIDE THE MAIN LOGIC 
    // WHICH CAN BE EASILY CHANGED and OTHER PARTS OF CODE REAMINS UNCHANGED 
    let guess = Guess::new(1232);
}
// <------------------------------------------------------------------------------------->