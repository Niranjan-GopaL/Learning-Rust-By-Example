//! This will contain all the stuff that are easily forgetable but useful
//! 
//! INLAY HINTS ARE SOO USEFUL FOR LEARNING

#![allow(non_snake_case)]

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
        if divisor == x {break }
        divisor += 1;
    }
    return true;
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
        print!("{:?}", i);
    }

    // here count and num is mutable but n is NOT [LOOK AT IT'S DESCRIPTION]
    let (mut count, n,mut num)  = (0,23,2);
    while count != n {
        if is_prime(num){
            count += 1;
            println!("{:?}",num);
        }
        num += 1;
    }
    // debug trait is not implemented for these. 
    // look at the error message for printing it
    // let t = (1,2,3,4,4,55,12,1,2,3,4,55,12);
    // println!("{:?}", t); 

}