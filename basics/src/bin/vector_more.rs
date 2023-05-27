//! Focuses on traversal in vectors
//! Introduces the concept of for loops 


fn main() {
    // Traversal

    let mut numbers = Vec::new(); 
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);

    for number in numbers {
        println!("{:?}", number);
    }

}