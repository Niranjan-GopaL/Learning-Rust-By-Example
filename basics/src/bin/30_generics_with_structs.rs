//! Structs and Ennums

#![allow(dead_code)]
#![allow(unused_variables)]

/// constrained x,y to i32
struct Point_ {
    x: i32,
    y: i32,
}

/// constrained x,y to u64
struct Point__ {
    x: u64,
    y: u64,
}

///
struct PointBest<T, U> {
    x: T,
    y: U,
}


/// constrained x,y same type T
struct PointBetter<T> {
    x: T,
    y: T,
}






fn main() {
    let p_weak = Point_ { 
        x: 0, 
        y: 0 
    };
    let p_weak = Point__ {
        x: 12424312341,
        y: 4123412341324,
    };

    // power of generics
    let point = PointBetter { 
        x: 123, 
        y: 1233
    };


    // DAMNNN
    let point = PointBest { 
        x: 123, 
        y: 12.312
    };

    // True generalisation lol
    let _point_can_be_anything = PointBest { 
        x: 123, 
        y: String::from("GOD THIS IS INSANE")
    };

    // We can have generics in enums too
    // In fact both Option and Result 
    // use this. Below is an overview of how it must be implemented internally

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    enum Option<T> {
        Some(T), // something is returned, we dont want to specify what is returned 
        None,
    } 



}
