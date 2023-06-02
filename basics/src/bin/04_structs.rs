fn activity() {
    enum Flavor {
        _Sweet,
        _Bitter,
        Hot,
        _Chilly,
        _Cold
    }


    struct Drink{
        flavor: Flavor,
        _ounce: f64
    }


    let drink1 = Drink{
        flavor : Flavor::Hot,
        _ounce: 32.3
    };

    fn print_data(drink:Drink) {

        // this is kinda wierd => Initially I wrote "let mut switch = 1" and compiler showed a warning
        // turns out just "let switch" is enough. MAKES SENSE THOUGH!!!!! damnnn
        let switch;

        let drink_flavor = drink.flavor;
        match drink_flavor {
            Flavor::_Sweet => switch = 0,
            Flavor::_Bitter => switch = 0,
            Flavor::Hot => switch = 0,
            Flavor::_Chilly => switch = 1,
            Flavor::_Cold => switch = 1
        };

        if switch == 1 
        {
            println!("Flavor is GOOD")
        } else 
        {
            println!("Flavor is BAD")
        }
    }

    print_data(drink1);

}


fn structures() {

    // Groups SIMILAR DATA . It's a blueprint 
    // to manufacture more data of similar kind
    struct FirstYear {
        _name:String,
        rollno:u32,
        _gender:char,
        _is_active:bool}

    // std_1 is a first year therefore would have structure of FirstYear
    let std_1 = FirstYear{
        _name : "Niranjan".to_string(),
        rollno:543,
        _gender:'M',
        _is_active:true};

    // getting FIELD_vALUE OF an "obj"
    let std_1_roll = std_1.rollno;
    print!("{std_1_roll}");

}

fn main() {
    structures();
    activity();
}