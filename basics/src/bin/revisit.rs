#![allow(dead_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]


enum TypePointer{
    Fast,
    Slow,
}

struct Node{
    val: i32,
    next : *const Node,
}

fn main() {

    println!("Hello World!");
    

    const ARR_LEN:usize = 5;
    let a:[i32;ARR_LEN] = [1,2,3,4,5];

    // for i in 1..ARR_LEN{
    //     let node[0] 
    // }

}
