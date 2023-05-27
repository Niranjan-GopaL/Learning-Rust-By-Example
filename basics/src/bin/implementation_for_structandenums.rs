struct Temperature{
    degree_f : f64
}


impl Temperature{

    fn freezing() -> Self{
       Self { degree_f: 0.0 }
    }
    
    /* THIS IS EQUALLY VALID. 
        cus "Self" is whatever is the "Name of Impl" 

    fn freezing() -> Temperature{
        Temperature { degree_f:0.0 }  
    }

    Why we don't use this 
     */

    fn show_temp(&self) {
        println!("{:?}", self.degree_f)
    }
}


fn show_temp(temp:Temperature){
    println!("{:?}", temp.degree_f)
}

fn main(){
    let hot = Temperature{degree_f: 99.9999};
    hot.show_temp();  // <-- impl
    show_temp(hot);   // <-- regular fn
    
    
    let cold = Temperature::freezing();
    cold.show_temp();  // <-- impl
    show_temp(cold);   // <-- regular fn
}