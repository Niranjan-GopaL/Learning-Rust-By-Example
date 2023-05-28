fn main() {
    let numbers = vec![10,20,30,40];
    
    // Initial try ===> Error 
    /*
    
    for i in numbers {               // main transfers ownership to for
        match i {
            30 => println!("thirty"),
            _ => println!("{}",i)
        }
    }

    println!("{}", numbers.len()); // <----------- using a moved variable's data

    */
    
    
    // Right way 

    for i in &numbers {
        match i {
            30 => println!("thirty"),
            _ => println!("{}",i)
        }
    };

    println!("{}", numbers.len());
}