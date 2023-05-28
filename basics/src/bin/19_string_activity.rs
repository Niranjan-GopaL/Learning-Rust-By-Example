//!                 Activity on Strings
//!     1) My way
//!     2) Better way

#[derive(Clone)]
struct _Person {
    name: String,
    age: usize,
    color: String,
}


impl _Person {
    fn print_name(name: &str) {
        println!("{:?}", name);
    }

    fn print_color(color: &str) {
        println!("{:?}", color);
    }
}



fn main() {

    // My way --------------------------------------------------->

    let people = vec![
        _Person {
            age: 12,
            color: "red".to_string(),
            name: "John".to_owned(),
        },
        _Person {
            age: 5,
            color: String::from("blue"),
            name: "Blake".to_owned(),
        },
        _Person {
            age: 9,
            color: String::from("pink"),
            name: "Alisa".to_owned(),
        },
    ];

    // since we have to give for loop the ownership, WHY NOT MAKE A CLONE AND GIVE
    // For some reason Copy impl cant be done with String.   reason :-----|
                                                                         //   
    /*                         WHY STRING DOES NOT HAVE Copy impl       ||           
    There's nothing wrong with having a String field in your type, but //
    it is incorrect/impossible to implement Copy for any type that contains a String.

    String can't implement Copy because (like Vec and any other variable-sized container), 
    it contains a pointer to some variable amount of heap memory. 
    The only correct way to copy a String is to allocate a new block of heap memory 
    to copy all the characters into, which is what String's Clone implementation does.
     */

    for person in people.clone() {
        let condition = person.age <= 10;
        match condition {
            true => {_Person::print_color(&person.color);        
                     _Person::print_name(&person.name);},
            false => ()
        };
    }
    


    // Better way ----------------------------------------------------->
    

    // Mostly what I wrote is correct. BUT ONE NEW THING!!!!!!!!!!!!!!
    for person in people {
        // since we need to check a conditon, IN THESE CASES `USE IF ELSE!!!!!`
        if person.age <= 10 {
          _Person::print_color(&person.color);        
          _Person::print_name(&person.name);
        };
    }

}
