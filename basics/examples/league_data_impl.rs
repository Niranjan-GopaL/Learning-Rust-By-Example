//! Big project to implement all the mathces that happen in all the leagues
//! and having custom data structures that can perform more functions

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

fn main(){

}