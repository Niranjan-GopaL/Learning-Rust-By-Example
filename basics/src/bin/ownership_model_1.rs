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

fn get_theme(theme: Theme) {
    match theme {
        Theme::_Dark => {
            println!("THE BEST THEMES ARE ALWAYS DARK ");
            println!("I love the fact that you can justdo  --->  something_in_math => {{ }} have sooo much deone in that expression")
        },
        Theme::_CutomisedForMe => {
            println!("I FUCKING LOVE THE CUSTOMISED VERSION i have now");
        },
        _ => (),
    }
}

fn main () {
    let my_theme = Theme::_CutomisedForMe; // <----- [[2]] my_theme is owned my main fn
    let _x = 1;                           // <-----       _x is also owned by main fn   
    let _color = "purple";
    get_theme(my_theme);  // <------------ [[3]] ownership is given to the other function
    // get_theme(my_theme);


} // <--------- [[4]]  all the data that function owns IS DESTROYED