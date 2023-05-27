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


fn main(){
    let (x,y) = (10,20);
    _module_function(x,y)
}