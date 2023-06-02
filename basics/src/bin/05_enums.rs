//! ENUMERATION

fn lesson_2() -> i32 {
    //  ALL POSSIBLE VARIENTS!!!!!!!!! <------- One way of thinkin
    // We created a "data-type-like thing", now varibales of specific 
    // data types belonging to another  broder class of data types can be created
    enum Direction {
        Up,
        Down,
        Right,
        Left,
        Front,
        Back,
    }

    fn get_direction_wrt_1_or_minus1(dir: Direction) -> i32 {
        // ALL VARIANTS MUST BE COVERED IN MATCH !!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        match dir {
            Direction::Up => return 1,
            Direction::Down => return -1,
            Direction::Right => return 1,
            Direction::Left => return -1,
            Direction::Front => return 1,
            Direction::Back => return -1,
        }
    }

    // AT A TIME AN enum can only be ONE of the varient

    // dir1 is of "Direction" data type <---------------- IMPORTANT
    let dir1 = Direction::Up;
    let dir2 = Direction::Down;
    let dir3 = Direction::Right;
    let dir4 = Direction::Left;
    let dir5 = Direction::Front;
    let dir6 = Direction::Back;

    println!("{}", get_direction_wrt_1_or_minus1(dir1));
    println!("{}", get_direction_wrt_1_or_minus1(dir2));
    println!("{}", get_direction_wrt_1_or_minus1(dir3));
    println!("{}", get_direction_wrt_1_or_minus1(dir4));
    println!("{}", get_direction_wrt_1_or_minus1(dir5));
    println!("{}", get_direction_wrt_1_or_minus1(dir6));

    return 0;
}

fn main() {
    lesson_2();
}
