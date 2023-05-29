//! Generics intro
//! there will be certain levels [[1]] and [[2]] [[3]]

#![allow(dead_code)]
#![allow(unused_variables)]


// [[2]] ------------------------------------------------------------------------------

/// largest i32 from Vec<i32>
fn largest_num(numbers:Vec<i32>) -> i32 {
    let mut largest = numbers[0];
    for num in numbers {if num > largest {largest = num;}};
    largest
}


/// largest char from Vec<char>
fn largest_char(numbers:Vec<char>) -> char {
    let mut largest = numbers[0];
    for num in numbers {if num > largest {largest = num;}};
    largest
}
// -----------------------------------------------------------------------------------


// Generic types [[3]] ----------------------------------------------------------------

/// This functino can return the largest quantity of from a Vec
/// be it i32 ,char, u32, String ,etc.
/// 
/// If the Vec items can be compared ===> this function works
fn largest_<T:PartialOrd + Copy>(list:Vec<T>) -> T {
    let mut largest = list[0];

    // binary operation (fancy way of ways all comparison op and stuff ><==! )
    // can't be done on T type (since both num and largest are of T type)

    // We restrict the general type T (it can be of ANY TYPE 
    // to a type T that :- has ` PartialOrd and Copy` traits
    // so that now T ---> any type T that hcan be compared
    for num in list {if num > largest {largest = num;}};
    largest
}



fn main(){
    // [[1]]
    let numbers = vec![1,2,3,4,5,6,67,8,9];
    let mut largest = numbers[0];
    for num in numbers {if num > largest {largest = num;}};
}