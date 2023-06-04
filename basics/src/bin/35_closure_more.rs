//! More examples on closure

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]


use std::thread;
use std::time::Duration;


/// expensive function that takes a lot of time and ersources
/// This fn calculates how much pushups , situps you have to do
fn simulated_expensive_calculations(intensity:u32)->u32 {
    println!("Calculating slowly ....");
    thread::sleep(Duration::from_secs(3)); // sleeps for 3 seconds
    intensity * 2 / 3 + 1
}




/// generate_workout() before closure
fn generate_workout(intensity:u32,random:u32)
{
    if intensity > 25 
    {
        // we do the same expensive function twice
        println!("Today you need to do {} pushups",simulated_expensive_calculations(intensity));
        println!("Today you need to do {} situps",simulated_expensive_calculations(intensity));
    }else 
    {
        if random == 3 {
            println!("Take a break today, remember to stay hydrated")
        }else {
            // we do the same expensive function
            println!("Today walk for {} minutes", simulated_expensive_calculations(intensity))
        }
    }

}




//<---------------------------------------------------------------------------------------------->

/// This is called a "Memoisation pattern" 
/// ====> Structs tht will - HOLD CLOSURE  - HOLD result of THAT closure
struct Cacher<T>
where T: Fn(u32) -> u32 
//  Closures HAS to implement one of these 3 similar trait bounds :- Fn, FnOnce, FnMut
//  Note normal functions can also implements these traits
{
    calculation:T, // this field stores the closure, [ we can store a regular function as well  ]
    value:Option<u32> // when we initialize the struct instance, value will be None BUT after that we'll sotre
}                     // BUT after that we'll store the result of that closure in value field  

//<---------------------------------------------------------------------------------------------->









fn main() {
    let simulated_intensity = 2;
    let simulated_rand_number = 2;



}