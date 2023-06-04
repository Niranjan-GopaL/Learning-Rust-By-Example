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














/// generate_workout() using closure
fn generate_workout_2(intensity:u32,random:u32)
{
    // closure. Logic is in a single place
    let simulated_expensive_calculations_result = 
    |intensity|
    {
        println!("Calculating slowly ....");
        thread::sleep(Duration::from_secs(3)); // sleeps for 3 seconds
        intensity * 2 / 3 + 1
    };

    if intensity > 25 
    {
        // we do the same expensive function twice
        println!("Today you need to do {:?} pushups",simulated_expensive_calculations_result(intensity));
        println!("Today you need to do {:?} situps",simulated_expensive_calculations_result(intensity));
    }else 
    {
        if random == 3 {
            println!("Take a break today, remember to stay hydrated")
        }else {
            // we do the same expensive function
            println!("Today walk for {} minutes", simulated_expensive_calculations_result(intensity))
        }
    }

}

























/// generate_workout() using closure and using Memoization patterns
fn generate_workout_3(intensity:u32,random:u32)
{
    let mut cached_result = // storing in the Cacher struct to use the memoization pattern
    Cacher::new(
    |intensity|
    {
        println!("Calculating slowly ....");
        thread::sleep(Duration::from_secs(3)); // sleeps for 3 seconds
        intensity * 2 / 3 + 1
    });


    if intensity > 25 
    {
        // we do the same expensive function twice
        println!("Today you need to do {:?} pushups",cached_result.value(intensity));
        println!("Today you need to do {:?} situps",cached_result.value(intensity));
    }else 
    {
        if random == 3 {
            println!("Take a break today, remember to stay hydrated")
        }else {
            // we do the same expensive function
            println!("Today walk for {} minutes", cached_result.value(intensity))
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


impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation:T) -> Self {
        Self{calculation, value: None}
    }



    fn value(&mut self,arg:u32) -> u32 {
        match self.value {
            Some(v) => v,
            // initialy when struct_instance.value(1234) is called
            // we pass 1234 to the calculation field which holds the closure
            // then value will store the closure's reuslt
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}


//<---------------------------------------------------------------------------------------------->









fn main() {
    let simulated_intensity = 200;
    let simulated_rand_number = 2;

    // compare how these perform :- (uncomment one and comment the rest)

    // generate_workout(simulated_intensity, simulated_rand_number);
    // generate_workout_2(simulated_intensity, simulated_rand_number);  
    generate_workout_3(simulated_intensity, simulated_rand_number);

}