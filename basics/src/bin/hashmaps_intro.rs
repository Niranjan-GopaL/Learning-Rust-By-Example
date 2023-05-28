use std::collections::HashMap;


fn main(){

    // Basic intro ----------------------------------------------------------------------------------̥

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    // declare a mutable empty hashmp
    let mut scores_hm = HashMap::new();

    // when inserting k,v pairs , we are actually
    // TRANSFERRING OWNERSHIP sigh. How to pass while retaining ownership
    // is discussed in the chapter 10 of rust lang book (using lifetimes
    scores_hm.insert(blue, 10);
    scores_hm.insert(yellow, 10);
    


    // get() <-- to get the values of specified keys --------------------------------------------------
    let team_name = String::from("Blue");

    // get() returns Option enum varient 
    // => either Some(something_that_you_need_to_extract) or None
    // I am extracting this after a lot of improvisation
    let _see_the_type_of_score = scores_hm.get(&team_name);

    let score =  match scores_hm.get(&team_name){
        Some(score) => *score,
        None => 0
    };

    println!("score of {} team is  {:?}",team_name,score);




    // insert() and or_insert() -----------------------------------------------------------------------------

    let mut hm = HashMap::new();

    // each time the value is overwritten
    hm.insert(String::from("blue"), 10);
    hm.insert(String::from("blue"), 20);
    hm.insert(String::from("blue"), 40);

    // hm.entry(...) returns an ENUM 
    // we have 4 impls on this ENUM and or_insert is the most important one!
    hm.entry(String::from("blue")).or_insert(10);
    // THIS LINE SAYS THAT if "blue" is an empty entry THEN INSERT VALUE INTO IT
    // IF ITS NOT => then DONT insert any value into it

    hm.entry(String::from("blue")).or_insert(213); 
    hm.entry(String::from("blue")).or_insert(1000000000); 
    hm.entry(String::from("blue")).or_insert(0); 
    // these dont do anything to the entry


    // VERY IMPORTANT POINT (WE MIGHT THIS IRLVERY OFTEN)
    // hm.entry(String::from("blue")).or_insert(10)  <-------- 
    //                      returns a mutable reference to the value of the entry
    // so we can dereference and mutate the value

    // The 10 line program hashmaps_use_case.rs does just that

}