//! THIS IS MODULE DOCUMENTATION



/// This is docstring for the this function
/// THIS LINE WILL ALSO BE SHOWN in documentation for this function
pub fn _module_function(x:i32,y:i32){
    println!("Hello world");
    
    // format string in rust is similar to python cuz
    // BOTH USES THE FMT library to format string
    let string_format = format!(" {x}*{y}={z} ",x=x,y=y, z = x*y);
    println!("{}",string_format);
}



// <--------------------------------------------------- TYPE ANNOTATION IN RUST ----------------------------------


enum Mouse{
    RightClick,
    LeftClick,
    MiddleClick
}

/// TYPE ANNOTATION IN RUST
fn type_annotation_importance(){
    println!("----------------type_annotation_importance----------------------");
    println!("Helps the Compiler tell what type is something");
    println!("We need to mannually type in the type only if compiler can't figure it out during run time");
    println!("GOod thing about RTUst is that it will tell us WHEN AND WHAT it can't figure out");

    // In these cases the compiler can figure what type a variable is
    let click = Mouse::RightClick;
    let a = 23;
    let character = 'a';
    // this is an important case  <--------------
    let clicks = vec![
        Mouse::RightClick,
        Mouse::LeftClick,
        Mouse::MiddleClick
    ];



    // During struct definition, IT IS IMPERTIE TO have type annotated
    struct UserPreference {
        frequent_clicks:Vec<Mouse>,
        speed:i32,
        
    }
    

    
    // During fn definition, IT IS IMPERTIE TO have type annotated
    fn get_user_pref(user_pref:UserPreference,sensitivity:i32,) -> () {


        // notice the fn funcName(param1:type, param2:type)->returnType
        // in RUST WE USE () in order to SIGNIFY NULL

    }

}

fn main(){
    let (x,y) = (10,20);
    _module_function(x,y);
    type_annotation_importance();
}