//! Structs 
//! Ennums
//! Impl 
//! with generic types
//! 
//! Generics are awesome because it reduces code duplication. 
//! THIS INSANELY ENHANCES PERFOMANCE. ( DRY at its finest - Dont Repeat Yourself )

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


/// constrained x,y same type T
struct PointBetter<T> {
    x: T,
    y: T,
}

/// this is just an implementation that use generic
/// NOTE THE SYNTAX IS DIFFERENT FROM A NORMAL impl
/// 
/// we specify the generic BOTH after impl and impl-name
impl<T> PointBetter<T> {
    // self is struct instance
    // we take a reference of struct instance and return 
    // A REFERENCE OF TYPE T

    /// x returns the reference of field x which is of type T
    fn x(&self) -> &T {
        &self.x
    }
}

// this is available only for those struct instances which
// which are of f64 type out of all the other types. The other impl
// was available for all types
impl PointBetter<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}




// <------------- this is soo FUCKING cool --------------


/// both can be even if different types
#[derive(Debug)]
struct PointBest<T, U> {
    x: T,
    y: U,
}

impl<T,U> PointBest<T,U>{
    /// basically we are defining a fn that works on :-
    /// 1 --> a struct instance. As the impl name implies, we are using these impl on 
    ///            struct instances that are of type T,U
    /// 2 --> we pass another struct instance that may be of different types <V,W> 
    /// 
    /// and then we give out a point that has the x from self, y from other 
    fn get_x_from_first_y_from_second<V,W> (self:PointBest<T,U>,other:PointBest<V,W>) -> PointBest<T,W> {
        PointBest{
            x:self.x,
            y:other.y,
        }
    }
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

    // impl with generic ----------------------

    let point_1 = PointBetter{x: 1, y:1};
    println!("{:?}", point_1.x()); // x impl is available for any types
    // println!("{:?}", point_1.y());  y is only available for those with f64 strcut instances

    let point_two = PointBest{x: String::from(""), y:1};
    let point_one = PointBest{x:'a' , y:123.123412};

    println!("{:?}", point_one);
    println!("{:?}", point_two);

    let point_three = point_one.get_x_from_first_y_from_second(point_two);

    println!("{:?}", point_three);
}
