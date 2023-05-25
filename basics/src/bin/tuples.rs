fn tuples() {}

fn activity() {
    fn get_3_nums_from_func() -> (i32, i32, i32) {
        (1, 2, 3)
    }

    // notice that here those wierd borrowing and ownership issues was not there
    let getting_many_nums_from_func = get_3_nums_from_func();
    let (a, b, c) = getting_many_nums_from_func;

    println!("{:?} {:?} ", a, getting_many_nums_from_func.0); // 1
    println!("{:?} {:?} ", b, getting_many_nums_from_func.1); // 2
    println!("{:?} {:?} ", c, getting_many_nums_from_func.2); // 3



    // WHY PEOPLE USE STRUCTS SOOOOOOO MUCH MORE THAN TUPLES :-

    // Tuples are used when the properties of a data are few in number
    // cuz reading tuple.0 tuple.1 is kinda ambigious

    let favourite = ("IU", 2, "blue", "rcb lollll", "Kerala", "Japan");

    print!("{:?}", favourite.0); // artist
    print!("{:?}", favourite.1); // no
    print!("{:?}", favourite.2); // team lolllllll
    print!("{:?}", favourite.3); // state
    // these are not obvious for a person who is reading this


    // STRUCT ROCKS
    struct Fav {
        artist: String,
        no: i8,
        team: String,
        state: String,
    }

    let fav_1 = Fav {
        artist: "John".to_string(),
        no: 8,
        team: "John".to_string(),
        state: "john".to_string(),
    };

    // MUCHHHHHHHHHHH MORE SENSE
    print!("{:?}", fav_1.artist); // artist
    print!("{:?}", fav_1.no); // no
    print!("{:?}", fav_1.team); // team lolllllll
    print!("{:?}", fav_1.state); // state


    // TUPLES ARE USEFUL WHEN SMALL DATA PROP ARE CONCEREND
    // OR 
}

fn main() {
    tuples();
    activity();
}
