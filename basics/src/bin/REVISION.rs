//! This will contain all the stuff that are easily forgetable but useful
//! 
//! INLAY HINTS ARE SOO USEFUL FOR LEARNING


#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
use std::fmt::Display;


fn is_prime(x: u32) -> bool {
    if x<=1 {
        return  false;
    }
    if x == 2 {
        return true;
    }
    let mut divisor:u32 = 2;
    loop {
        if x%divisor == 0 {return  false}
        if divisor == x-1 {break }
        divisor += 1;
    }
    return true;
}



// <--------------------------THIS IS THE EXACT SYNTAX OF IMPL-----------------------------------------------
struct Point<T> {
    x: T,
    y: T,
}
        

/// the T near impl DECIDED what all are the characteristics of the generic type that 
/// this implementation accepts and the TYPE FOR WHICH /"POINT"\it impls, just "accepts it" 
impl<T:Display+PartialOrd>  Point<T>{
    /// we are able to both display and compare cuz we accepts only THOSE T which 
    /// can impl both display and compare
    fn compare_and_display(x:T, y:T) {
        if x > y{
            println!("{}", x);
        }else{
            println!("{}", y);
        }
    }
}


fn main(){
    let l = (0..10,1,2,"This is STRING and not &str".to_owned());
    println!("{:?}", l.0);
    println!("{:?}", l.1);
    println!("{:?}", l.3);

    //  YOU ALWAYS FORGET THAT WE CAN DESTRUCTURE TUPLES
    let (a,_,_,_) = l;
    println!("{:?}", a);

    for i in a{
        println!("{:?}", i);
    }

    // here count and num is mutable but n is NOT [LOOK AT IT'S DESCRIPTION]
    let (mut count, n,mut num)  = (0,23,2);

    while count != n {

        if is_prime(num){
            count += 1;
            print!("{:?} ",num);
        }
        num += 1;

    }
    // debug trait is not implemented for these. 
    // look at the error message for printing it
    // let t = (1,2,3,4,4,55,12,1,2,3,4,55,12);
    // println!("{:?}", t); 

    // THERE ARE TWO LOOPS => loop{} and 
    //                      =>  while condition{}

    // to manufacture more data of similar STRUCTURE => struct

    
    /*     format :- 

        match <> {
            first expression,
            second  expression,
            third  expression,
            fourth expression,
        }
    */

    /*  IMPORTANT THING TO NOTICE

        match some_no {
        1 => println!("{some_no}"),
        2 => println!("{some_no}"),
        3 => println!("{some_no}"),
        4 => println!("{some_no}"),
        other => println!("Big")                     //<----------- REMEMBER THIS PRACTISE

        THE SET OF KEYS MUST BE EXHAUSTIVE and  each must be UNIQUE !!!!!
        THAT MEANS YOU CAN'T DO SOMETHING LIKE :-

        match some_no {
        some_no > 100 => println!("{some_no}"),
        some_no = 100 => println!("{some_no}"),
        some_no < 100 => println!("{some_no}"),
        }

        eventhough we exhaust all possibilites of some_no , 
        the left hand side is {F,T,F} and therefore not unique

     */



}