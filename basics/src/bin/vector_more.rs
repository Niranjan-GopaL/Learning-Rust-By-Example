//! Focuses on traversal in vectors
//! Introduces the concept of for loops 


/// to furthur explain how vectors are powerful
struct Exam{
    bio:u32,
    chem:u32,
    phy:u32,
    history:u32,
    geography:u32,
}

fn main() {
    // Traversal

    let mut numbers = Vec::new(); 
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);



    println!("{:?}", numbers);  //<------- works here
    
    // loop specifically more collections
    // we transfer ownership from main to "for loop"
    for number in numbers {
        println!("{:?}", number);
        
    }

    // println!("{:?}", numbers);  //<------- won't work here


    // Vector of Structs 
    let my_exams = vec![
        Exam{ bio:100, chem:100, phy:100, history:100, geography:100,},
        Exam{bio:69,chem:79,phy:92,history:89,geography:70,},
        Exam{bio:69,chem:67,phy:100,history:50,geography:80,}
    ];

    
    // println!("{:?}",my_exams[2].phy);
    for exam in my_exams{
            println!("{:?}",exam.bio);
            println!("{:?}",exam.chem);
            println!("{:?}",exam.phy);
            println!("{:?}",exam.history);
            println!("{:?}",exam.phy);
            println!("{:?}",exam.geography);
    };

}