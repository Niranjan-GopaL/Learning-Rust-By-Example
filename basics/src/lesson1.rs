pub fn lesson_1() -> i32 {
    let mut a = 0;
    
    loop{
        if a == 5{
            break
        }
        print!("{a}");
        a = a + 1;
    }
    
    let mut b = 0;
    while b != 5 {
        print!("{b}");
        b = b + 1;
        
    }


    fn first_name() -> String {"Niranjan".to_string()}
    fn last_name() -> String {"Gopal".to_string()}

    println!("{} {}",first_name(),last_name());

    return 0
}