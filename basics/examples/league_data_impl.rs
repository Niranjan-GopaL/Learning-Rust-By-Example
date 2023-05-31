//! Big project to implement all the mathces that happen in all the leagues
//! and having custom data structures that can perform more functions

use std::fs::{self,File};
use std::io;
enum Postion{
    Stricker,
    Goal,
    Forward,
    Defender,
    MidFielder,
}

struct Player{
    name: String,
    rank: i32,
    position: Postion,
}

struct Team{
    name: String,
    rank: i32,
    members: Vec<Player>
}

struct League{
    name: String,
    popularity: i32,
    teams: Vec<Team>
}

// have a hashtable to store the each league's data
// how many matches they played , how many they won,
// how many points they have.









fn read_from_file(paths: Vec<String>)/*->Result< Vec<String>,io::Error>  */ {
    /*
        paths[0] => "/path/to/League_1.txt"
        paths[1] => "/path/to/League_2.txt
        paths[2] => "/path/to/League_3.txt
     */
    let f1_as_string = fs::read_to_string("League_1.txt");
    let f2_as_string = fs::read_to_string("League_2.txt");
    let f3_as_string = fs::read_to_string("League_3.txt");

    // return vec![f1_as_string, f2_as_string, f3_as_string];
}



fn main(){
}