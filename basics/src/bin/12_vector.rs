//! More about vectors in Rust



fn main(){
    // Unlike arrays and tuples, vectors allocate memory on heap
    
    
    
    // Basic example

    // vec! is a macro (so it expands into actual code during compile time)
    // so something similar to vector_2's push would be done here
    let _vector_1 = vec![1,2,3,4,5]; // does not matter if its immutable 

    // Vec is a struct (hover over it) that allocates memory on heap
    let mut vector_2 = Vec::new(); // needs TO BE MUTABLE
    vector_2.push(1);
    vector_2.push(2);
    vector_2.push(3);
    vector_2.push(4);

    vector_2.pop();
    println!("{:?}",vector_2.len());


    // the problem with accessing the vector's index this way is that
    // out of bounds error can't be checked at compile time. 
    // USE get() instead
    println!("{:?}",vector_2[0]);
    println!("{:?}",vector_2[1]);
    println!("{:?}",vector_2[2]);
    // println!("{:?}",vector_2[3]); //<----- a runn time error

    println!("{:?}",vector_2.get(0));
    println!("{:?}",vector_2.get(1));
    println!("{:?}",vector_2.get(2));
    // println!("{:?}",vector_2.get(3)); //<----- THIS WON'T COMPILE YAY!!!



}