enum Access {
    _Admin,
    _Guest,
    _Manager,
    _User
}

fn main(){
    let access_level = Access::_User;
    let is_access = match access_level {
        Access::_Admin => true,
        _ => false,
    };
    println!("{is_access}");

}