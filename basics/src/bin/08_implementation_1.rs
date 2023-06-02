struct Temperature{
    degree_f : f64
}


impl Temperature{
    fn _new_freezing() -> Self{
       Self { degree_f: 0.0 }
    }
    
    /* THIS IS EQUALLY VALID. 
        cus "Self" is whatever is the "Name of Impl" 

    fn _new_freezing() -> Temperature{
        Temperature { degree_f:0.0 }  
    }

    Why we don't use this 
     */

    fn show_temp(&self) {
        println!("{:?}", self.degree_f)
    }
}

fn show_temp(temp:&Temperature){
    println!("{:?}", temp.degree_f)
}

fn main(){
    let hot = Temperature{degree_f: 99.9999}; // <--- creating a struct instantce normally
    hot.show_temp();  // <-- impl
    show_temp(&hot);   // <-- regular fn
    
    let cold = Temperature::_new_freezing();  // <--- creating a struct instantce normally
    cold.show_temp();  // <-- impl
    show_temp(&cold);   // <-- regular fn

    // both srtuct instaces are available in the main (we only evr let functions and impl borrow)
    println!("{:?}",hot.degree_f);
    println!("{:?}",cold.degree_f);
}