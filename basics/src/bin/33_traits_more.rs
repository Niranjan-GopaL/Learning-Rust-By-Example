//! [1] Trait bounds , generics  
//! [2] Syntax-sugaring 
//! [3] Return type can be a type that implements certain traits [IMP for closyres and iterators] 
//! [4] A nice quirk 
//! [5] A good example

struct Foo1{
    bar1: String,
    bar2: String,
    bar3: String,
}

impl Speak_bar for  Foo1 {
    fn speak_bar(&self) -> String{
        // concat!("I GOT THIS FROM THE RUSTLANG DOCS",String::from(&self.bar1)).to_owned()
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

impl Speak_bar for  Foo2 {
    // it can still impl the DEFAULT behavior of the trait fn 
}

trait Speak_bar{
    fn speak_bar(&self) -> String{
     String::from("THI IS THE DEFAULT IMPL")
    }
}

// <-------------------------------------------------------------------------


fn notify_1(param:&impl Speak_bar) 
{   
    
}


fn notify_2<T:Speak_bar>(param:T) 
{

}

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

}