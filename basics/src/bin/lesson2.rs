fn lesson_2() -> i32 {

    // most important thing about MATCH :-  THEY MUST BE EXHAUSTIVE
    // all possibilities MUST BE ACCOUNTED FOR in matching
    let some_bool = true;
    let some_no = 123;

    // THESE ARE FAT ARROWS. LOLLLLLLLLLLLLLLLL!
    //  matches evaluates EXPRESSION AND NOT STATEMENTS (hence the comma after each match)

/*     format :- 

    match <> {
        first expression,
        second  expression,
        third  expression,
        fourth expression,
    }
 */

    match some_bool {
        // true => println!("{some_bool}")  <--- first expression 
        true => println!("{some_bool}"),
        // false => println!("{some_bool}")  <--- second expression 
        false => println!("{some_bool}"),
    }


    //  how to account for all possibilites
    match some_no {
        1 => println!("{some_no}"),
        2 => println!("{some_no}"),
        3 => println!("{some_no}"),
        4 => println!("{some_no}"),
        // anything else will give this
        _ => println!("Big")
    }


    // If you added a new possibility while coding compiler will intstruct that
    // you missed a case . So its helpful in writing clean code . 
    // Compiler wont check if else


    /*  IMPORTANT THING TO NOTICE

        match some_no {
        1 => println!("{some_no}"),
        2 => println!("{some_no}"),
        3 => println!("{some_no}"),
        4 => println!("{some_no}"),
        _ => println!("Big")

        THE SET OF KEYS MUST BE EXHAUSTIVE AND EACH MUST BE UNIQUE !!!!!
        THAT MEANS YOU CAN'T DO SOMETHING LIKE :-

        match some_no {
        some_no > 100 => println!("{some_no}"),
        some_no = 100 => println!("{some_no}"),
        some_no < 100 => println!("{some_no}"),
        }

        eventhough we exhaust all possibilites of some_no , 
        the left hand side is {F,T,F} and therefore not unique

     */


    return 0;
}


fn main() {
    lesson_2();
}