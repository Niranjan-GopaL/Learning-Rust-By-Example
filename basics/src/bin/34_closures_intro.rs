//! Closures are anonymous fn ,
//!         - can be stored as variables
//!         - passed around
//!         - can be passed as input parameters to other fn
//!         - unlike fn, they can USE THE variables in their environment
//!         - for fn definition we had to explicitly define the types
//!           we don' have to for closures.
//!
//!                 Compiler will infer the type WHENEVER THE CLOSURE IS FIRST CALLED
//!                 but after that the type would be concrete, meaning the type infered
//!                 in the beginning is FINAL.
//!
//! Remember the variable would store CLOSURE ITSELF and not it's return value
//! 
//!
//!
//! Closures can capture values from their environment in THREE WAYS, 
//! which directly map to the three ways a function can take a parameter:
//!         -   borrowing immutably
//!         -   borrowing mutably
//!         -   taking ownership




#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]



#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}


struct Inventory {
    // shirts: vec!["ShirtColor::Red", "ShirtColor::Blue","ShirtColor::Blue","ShirtColor::Red"]
    shirts: Vec<ShirtColor>,
}


impl Inventory {

        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor
        {
            // closures are beautiful
            // unwrap_or_else() UNWRAPS the Option enum  => "Returns the contained Some value or computes it from a closure"
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor
        {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                }
            }

            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
}



fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
    "The user with preference {:?} gets {:?}",
    user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
    "The user with preference {:?} gets {:?}",
    user_pref2, giveaway2
    );

}