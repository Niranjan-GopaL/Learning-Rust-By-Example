//! entry point
#![allow(non_snake_case)]

extern crate ncurses;
use ncurses::*;




fn main() {

    initscr();

    let mut quit = false;
    
    let mut to_do = vec![
        "execute the plan",
        "I am happy",
        "Make tea",
        "Learn Cpp",
        "Learn more on rust",
        "Learn more twcss",
        "Learn more MUI",
    ];

    // INDEX at which cursor is there̥
    let to_do_curr:usize= 0;
    
    
    while !quit {
        
        for (row,todo)  in to_do.iter().enumerate() {
            mv(row as i32, 0);
            addstr(*&todo);
        }
        
        refresh();
        
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            other => println!("{}",other)
        }
    }
    
    endwin();
}
