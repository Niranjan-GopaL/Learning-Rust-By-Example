//! String types in Rust =>
//! String  -> String data type 
//! &str    -> are string slices


/* KNow these stuff about strings:- 

    1. Strings are automatically borrowed 
            ( i.e any of this type is -> "THIS IS  STRING SLICE" t
            here fore borrowed by default )

    2. Two ways to "own copy of this string slice" 
            i. let owned_string = "THIS IS A SLICE BYT YOU CAN OWN IT NOW".to_owned()
            ii.let owned_string =  String::from("THIS IS A SLICE BYT YOU CAN OWN IT NOW")
            
    3. Inside struct declaration USE OWNED STRINGS
            
            
*/



// We need to use Owned String inside Struct definition

// struct ThisIsWrong{
//     name:&str
// }




/// Correct way
struct ThisIsRight{
    name : String,
}

/// accepts a string ref
fn accepts_string(s: &str) {
    println!("{:?}",s);
}


fn main() {

    // [[ PART ONE  ]]
    let string_slice: &str = "THIS IS A STRING SLICE";
    
    let owned_string_1: String = "THIS IS A SLICE BUT YOU CAN OWN IT NOW".to_owned();
    let owned_string_2: String =  String::from("THIS IS A SLICE BUT YOU CAN OWN IT NOW");
    
    
    accepts_string(&string_slice);
    accepts_string(&string_slice);
    accepts_string(&string_slice);
    
    // primitve types' ownership are not transferred
    accepts_string(&owned_string_1);
    accepts_string(&owned_string_1);
    accepts_string(&owned_string_1);
    
    
    // primitve types' ownership are not transferred
    accepts_string(&owned_string_2);
    accepts_string(&owned_string_2);
    accepts_string(&owned_string_2);
    
    
    


    // [[ PART TWO  ]]
    
    

    // You can't pass string slice as a field to STRUCT.
    // AS SAID EARLIER => ONLY OWNED STRING CAN BE PASSED 

    // let struct_instance_1 = ThisIsRight{
    //     name: string_slice,
    // }; 
    
    
    // CORRECT WAY :- 

    let struct_instance = ThisIsRight{
        name : owned_string_1
    };
    




    // [[ PART THREE  ]]

    // JUST LIKE FUNCTION CALLS , WE ARE PASSING THE OWNERSHIP DURING STRUCT ISNTANCE CREATION
    let struct_inst =  ThisIsRight{
        name: owned_string_2,
    }; //<------ owned_string_2 is no more available

    // accepts_string(&owned_string_2); <------- value moved can't be borrowed


}