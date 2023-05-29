use std::fs::File;
use std::io::ErrorKind;

// For demonstarating the error propogation
use std::io::Read;
use std::io;

// For writing one line function
use std::fs::{self};










// just like how the Option Enum can be either Some or None
// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }
// many functions RETURN an enum varient

// this exact enum with lots of impl functions are included in the prelude 
// DAMNN!!!



fn main() {

    // clearly the inlay hints tell us `_f` is Result enum varient that returned by open()
    let _f = File::open("hello.txt");

    // this is sooo nice
    let _f = match _f{
        // NOTICE these care Ok and not Result::Ok()
        // this is the built-in enum varient, not ours
        Ok(_f) => _f,
        Err(e) => panic!("Problem opening file, error : {}", e)
    };




    // since hello.txt DNE, it'll panic out. But if we want to create a new file 
    let _f = File::open("hello.txt");
    let _f = match _f {
        Ok(_f) => _f,
        Err(e) => match e.kind() {
                    ErrorKind::NotFound => match File::create("hello.txt") {
                                                Ok(newfile) => newfile,
                                                Err(e) => panic!("Problem while creating a file, error : {:?}", e),
                                            },
                    other_error => panic!("Problem opening the file, error : {:?}", other_error)
                    }
        };



        // Error Propogation philosphy 
        
        // Whenever a function calls another function_2 which can potentially give 
        // error, the way error should be handled SHOULD BE DETERMINED BY CALLER
        
        let _user_name = _read_username_from_file();
        let _user_name = _read_username_from_file__();
        let _user_name = _read_username_from_file_insane();
        
}
    





/// IMPORTNAT THING I LEARNT WHEN WRITING FNs in RUST IS
/// --> JUST KEEP ON WRITING, YOU'LL GET ERROR BUT ONCE YOU FINISH WRITING THE FN
/// YOU CAN HAVE THE COMPILER HELP YOU EXACTLY IN WHAT YOU WANT TO DO
/// 
/// so do NOT DEBUG IN MIDDLE OF WRIITNG A FN 
fn _read_username_from_file() -> Result<String,io::Error> {
    let _f = File::open("user_name.txt");
    
    // no idea why _f has to be mut but compiler told me so
    // I found out why --> read_to_string() accepts a mutable reference to file THAT'S WHY  
    let mut _f = match _f {
        Ok(_f) => _f,
        Err(e) => return Err(e),
        //mismatched types
        // expected unit type `()`
        // found enum `std::result::Result̥<_, std::io::Error>`̥

        // Therfore YOU NEED TO SPECTIFY THE RETUR TYPE OF FN 
        // at this point fn seems to return Result<_,io::Error>

        // actually just like said in the fucntion docstring 
        // you don't have to worry about errors now, COMPLETE THE FN then debug
    };

    let mut s = String::new();

    // notice that there is no semicolon. THis is what the entire fn
    // evaluates to
    match _f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}






// <--------------- awesome version -----------------------------

/// Better version of read_username_from_file()
fn _read_username_from_file_()->Result<String,io::Error>  {
    
        //` ? operator`
        // used in a function that returns `Result` or `Option` 
        // (or another type that implements `FromResidual`)

        // `?` means 2 possibilities (actually its just  the replacement for the mannual match we did)

        //  1 -> returns whatever the fn returns 
        //  2 -> makes the function return with the error it gave 
        //      (it does not panic rather lets the caller decide what to do with the error)
        
        
        let mut f = File::open("username.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
        
    }    
// --------------------------------------------------------------------------    
    
    
    








    
// <--------------- INSANE  version -----------------------------


/// 2 line _read_username_from_file()
/// Simplified using chaining function calls
fn _read_username_from_file__() ->Result<String,io::Error>  {
    let mut s = String::new();
    
    // on looking carefully we understand that we dont really need 
    // a file variable . we can just chain them together
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}    
// --------------------------------------------------------------------------







// <--------------- 1 LINE VERSION  version -----------------------------

/// We use std::io::fs::{self} for this
/// fs has a REALLY useful fucntion called :- 
/// read_to_string()
fn _read_username_from_file_insane() ->Result<String,io::Error>  {
    fs::read_to_string("path/to/your/file.txt")
}       

//<-----------------------  MIC DROP -------------------------------------


// NOTICE WE ARE USING read_to_string() that are actually TWO DIFFERENT METHODS
// They come from different module .ITS A LITTLE CONFUSING SINCE THEY HACE THE SAME NAME