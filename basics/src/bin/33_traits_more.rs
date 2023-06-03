//! [1] Trait bounds , generics  
//! [2] Syntax-sugaring 
//! [3] Return type can be a type that implements certain traits [IMP for closyres and iterators] 
//! [4] A nice quirk 
//! [5] A good example

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Display, Debug};




struct Foo1{
    bar1: String,
    bar2: String,
    bar3: String,
}

impl SpeakBar for  Foo1 {

    fn speak_bar_trait(&self) -> String{
        // ["hello", "world"].join(" ") 

        // MY FIRST FEVER FINDING ON MY OWN FROM DOC
        ["MY FIRST FEVER FINDING ON MY OWN FROM DOC".to_string(),String::from(&self.bar1)].join(" ")
    }
}

struct Foo2{
    bar1: String,
    bar2: String,
    bar3: String,
}

impl SpeakBar for  Foo2 {
    // it can still impl the DEFAULT behavior of the trait fn 
}







/// Traits need UpperCamelCase
trait SpeakBar{
    fn speak_bar_trait(&self) -> String{
     String::from("THI IS THE DEFAULT IMPL")
    }
}



// <-------------------------------------------------------------------------

/// this means param is some type that has SpeakBar trait {  "that immplements SpeakBar trait" }
fn notify_1(param:&impl SpeakBar) 
{   
        // THIS IS THE "SYNTAX-SUGARED" WAY OF below
}

/// THIS IS THE REA DEAL. takes in a generic type that as SpeakBar trait
fn notify_2<T:SpeakBar>(param:T) 
{
    
}
// <-------------------------------------------------------------------------






// <-------------------------------------------------------------------------

fn func1<T:>(param1:T, param2:T) 
{
    // this means that both takes in a generic type that implements SpeakBar trait
    // AND THAT both params are of same type
}


fn func2(param1:&impl SpeakBar, param2:&impl SpeakBar) 
{
    // this means that both takes in a type that implements SpeakBar trait
    // BUT BOTH CAN BE OF DIFF TYPE 
}
 // <-------------------------------------------------------------------------
 
 
 
 
 // <-------------------------------------------------------------------------

/// Returns some type that has a specific trait
fn make_speakbar_able() -> impl SpeakBar {
    Foo1{
        bar1: "bar1_of_Foo_1".to_string(),
        bar2: "bar2_of_Foo_1".to_string(),
        bar3: "bar3_of_Foo_1".to_string(),        
    }
}
 
 
 
// <------------------------------------------------------------------------
 struct Point<T> {
     x: T,
     y: T,
    }

    /// refresh this syntax. Self is WHATEVER name of impl ==> Point<T>
    impl<T> Point<T>{
        fn new(x:T,y:T) -> Self {
        Self 
        {   x, 
            y
        }
    }
}


impl<T:Display+PartialOrd>  Point<T>{

    /// we are able to both display and compare
    fn compare_and_display(x:T, y:T) {
        if x > y{
            println!("{}", x);
        }else{
            println!("{}", y);
        }
    }
}

// <------------------------------------------------------------------------







/*                      We'll see later

WE CAN IMPLEMENTE TRATI FOR TRAITS

                    impl <T: Clone> Debug for T{
                        // - stuff -
                    }

 */


fn main(){
    let foo_1 = Foo1{
        bar1: "bar1_of_Foo_1".to_string(),
        bar2: "bar2_of_Foo_1".to_string(),
        bar3: "bar3_of_Foo_1".to_string(),
    };

    let foo_2 = Foo2{
        bar1: "bar1_of_Foo2".to_string(),
        bar2: "bar2_of_Foo2".to_string(),
        bar3: "bar3_of_Foo2".to_string(),
    };

    println!("{:?}",foo_1.speak_bar_trait());
    println!("{:?}",foo_2.speak_bar_trait());

}