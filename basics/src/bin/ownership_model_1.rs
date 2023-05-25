// main things to know (NUMBERED):-

// [[1]]  wherever you create a variable that PLACE become the owner
//        of that varaible

enum Theme {
    _Dark,
    _Light,
    _CutomisedForMe,
    _Boring,
    _Funky
}

fn get_theme(theme: Theme) {  // <--------- [[4]] get_theme() function gets the ownership of my_theme
    match theme { // <-- funtion does whatever it wants to do with my_theme (now as theme)
        Theme::_Dark => {
            println!("THE BEST THEMES ARE ALWAYS DARK ");
            println!("I love the fact that you can justdo  --->  something_in_math => {{ }} have sooo much deone in that expression")
        },
        Theme::_CutomisedForMe => {
            println!("I FUCKING LOVE THE CUSTOMISED VERSION i have now");
        },
        _ => (),
    }
} // <------------- [[5]] get_Theme DELETES theme (my_theme) therefore that variable no longer exists


fn _main_ () {
    let my_theme = Theme::_CutomisedForMe; // <----- [[2]] my_theme is owned my main fn
    let _x = 1;                           // <-----       _x is also owned by main fn   
    let _color = "purple";
    get_theme(my_theme);  // <------------ [[3]] ownership of that variable is given to the other function
    // get_theme(my_theme);// <------------ [[6]] ERROR because that variable does not exist anymore

    
} // <--------- [[7]]  all the data that function main owns IS DESTROYED














// SOLUTION -----------------



fn _get_theme_solution(theme: &Theme) {  // <--------- [[4]] _get_theme_solution() function BORROWS variable nad processes it
    match theme { // <-- funtion does whatever it wants to do with my_theme (now as theme)
        Theme::_Dark => {
            println!("THE BEST THEMES ARE ALWAYS DARK ");
            println!("I love the fact that you can justdo  --->  something_in_math => {{ }} have sooo much deone in that expression")
        },
        Theme::_CutomisedForMe => {
            println!("I FUCKING LOVE THE CUSTOMISED VERSION i have now");
        },
        _ => (),
    }
    
} // <------------- [[5]] gives the variable back to main as _get_theme_solution was never the OWNER of it 
  // BUT THOSE VARIABLE whose owner was _get_theme_solution() . THOSE ARE DESTROYED !!!!!







// remove the underscore here and add the underscore in the other main 
// HOW TO SWITCH BETWEEN THE ENTRY POINT TO A PROGRAM !!!!
fn main () {
    let my_theme = Theme::_CutomisedForMe; // <----- [[2]] my_theme is owned my main fn
    let _x = 1;                           // <-----       _x is also owned by main fn   
    let _color = "purple";
    _get_theme_solution(&my_theme);  // <------------ [[3]] _main_() gives REFERENCE of my_theme to _get_theme_solution(). OWNER IS STILL main funciton
    _get_theme_solution(&my_theme);// <------------ [[6]] No error as the variable was RETURNED TO THE OWNER after use

} // <--------- [[7]]  all the data that function main owns IS DESTROYED