//! Matching on structs and enums (in a better way)
#![allow(dead_code)]
#![allow(unused_variables)]


enum Discount{
    Percent(f64),
    Flat(u64)
}

struct Ticket{
    event: String,
    price: i32,
    location: String,
}

fn main(){

    // --------------- NEW TECHNIQUE ---------------
    
    // THis is how we did it till now
    let n = 3;
    match n{
        3 => {},
        // we use _ to neglect stuff that we don't want
        _ =>{}
    }

    // But often times we might want to do something with it.
    // So correct way to do it is like this:- 

    match n{
        3 => {},
        other=>{
            // now we can use other to refer to this case,
            // which is more convenient than having a _
            
            print!("{other}");
        }
    }



    // --------------- NOW IN ENUMS AND STRUCTS  -----------------------------


    // First on enums. Those enums which can have multiple data varient can be match like this:-

    /*

    enum Discount{
        Percent(f64),
        Flat(u64)
        }


    So we can match stuff while considering the posibilities of the data that varient would have

    let enum_instance = Discount::Percent(12.1324);
    match enum_instance{
        Discount::Percent(percent) => {},
        Discount::Flat(amount) => {}
    }
    
    ITS SOOOOO FUCKING AWAESOME
    */

    let flat_discount = Discount::Flat(120);
    match flat_discount {
        // flat_discount is of type "Discount" so there are two possible Varient

        // 1.  (i)  the first varient is Discount::Flat
        //     (ii) we can name "amount" so that we can use amount in the 
        //          expression in the match arm
        Discount::Flat(amount) => {
                        if amount > 500 {
                            println!("You got insanly lucky with {} as discount", amount);
                        } else {
                            println!("You got lucky with {} as discount", amount);
                        }
                        },


        // 2. (i)  the second varient is Discount::Percent but HERE I AM JUST HAVING FUN LOL
        //    (ii) I am just goofing around here lol
        other => {
                        match other {
                            Discount::Percent(percent) => {
                                                if percent > 50.00 {
                                                    println!("You got insanly lucky with {} as discount", percent);
                                                } else {
                                                    println!("You got lucky with {} as discount", percent);
                                                }
                                                },
                            other_something => ()
                        }
                        }
    }

    let struct_instance = Ticket{location:String::from("Bangalore"),event:"Some stuff god knows".to_owned(),price:10000};
    let location_ = String::from("Bangalore");
    // THIS IS NEW!!!!!!!!!!!!!!!!!!!!
    match struct_instance{
        // use the TWO DOTS TO ignore the rest of fields

        Ticket{price:500,event, ..} => println!("{:?} event is available at 50 rupee", event),
        Ticket{price:100,event ,..} => println!("{} event is available at price 100", event),
        Ticket { location:location_,price,..} => println!("There is an event in Bangalore at {}",price)
    };

}   