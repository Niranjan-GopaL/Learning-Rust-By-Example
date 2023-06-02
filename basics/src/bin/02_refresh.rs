enum Access {
    _Admin,
    _Guest,
    _Manager,
    _User
}

fn main(){
    let access_level = Access::_User;
    
    let can_admin = match access_level {
        Access::_Admin => true,
        _ => false,
    };
    println!("{can_admin}");
}