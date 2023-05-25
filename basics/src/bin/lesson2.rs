fn lesson_2() -> i32 {

    // most important thing about MATCH :-  THEY MUST BE EXHAUSTIVE
    // all possibilities MUST BE ACCOUNTED FOR in matching
    let some_bool = true;

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

    return 0;
}


fn main() {
    lesson_2();
}