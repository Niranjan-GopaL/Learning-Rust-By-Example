use std::fs::File;
use std::io::ErrorKind;

// For demonstarating the error propogation
use std::io::Read;
use std::io;

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

}

fn _read_username_from_file() -> Result<String,io::Error> {
    let _f = File::open("user_name.txt");

    let mut _f = match _f {
        Ok(_f) => _f,
        Err(e) => return Err(e),
        //mismatched types
        // expected unit type `()`
        // found enum `std::result::Result̥<_, std::io::Error>`̥

        // Therfore YOU NEED TO SPECTIFY THE RETUR TYPE OF FN BEFOREHAND
        // at this point fn seems to return Result<_,io::Error>
    };

    let mut s = String::new();

    match _f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}