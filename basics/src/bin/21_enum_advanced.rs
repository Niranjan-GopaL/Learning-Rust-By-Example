//! ENUMS IN ALL ITS GLORY (I HOPE!! LOLL)

/*                 KNOW THESE 3  POINTS ABOUT ENUMS :-

    1. variables can be only ONE OF THE enum VARIENT at a time 

    2. enum VARIENTS can optionally contain MORE DATA 
       (if a varieant is another enum or struct or something else)

    3. Within the same enum there can be :-
        i)  plain identifier varients
        ii) data-containing varients


 */



#[derive(Debug)]
enum PromoDiscount{
    NewUser,
    Holiday(String)
}



// [[2]] and [[3]] is highlighted here --------------

/// This enum has 3 varients that are plain identifiers 
/// and 1 varient that is a "data containing" varient
#[derive(Debug)]
enum Discount{
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String)
}



fn main() {

    // defining variables with Advnaced Enums Variants
    // [[1]]
    let discount_1 = Discount::Percent(13.21233); // 13.2 percent discount  
    let discount_2 = Discount::Flat(120);         // 120 rupee flat discount  

    let discount_3 = Discount::Custom("CXASDFAER1341".to_owned());  // discount Coupon
    let discount_4 = Discount::Promo(PromoDiscount::NewUser);       // NewUser PROMOTIONAL DISCOUNT
    let discount_5 = Discount::Promo(PromoDiscount::Holiday(String::from("Christmas"))); // CHRISTMAS HOLIDAY PROMOTIONAL DISCOUNT

    //  we cant print cuz of the debug impl using derive macro
    println!("{:?}", discount_1);
    println!("{:?}", discount_2);
    println!("{:?}", discount_3);
    println!("{:?}", discount_4);
    println!("{:?}", discount_5);

}