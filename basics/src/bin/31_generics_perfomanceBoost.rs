//! Why generics enfances perfomance:- 
//! 
//! Here during compile time,
//! compiler splits this ```ONE option enum into two (or as many as you want )```
//! 
//! one for i32 and one for f64


enum _Option<T> {
    Some(T),
    None
}
/* At compile time the enum above will split into the two enums below

enum _Option<i32> {
    Some(i32),
    None
}
enum _Option<f32> {
    Some(f32),
    None
}
 */

fn main() {

    let _val = _Option::Some(12431);
    let _value = _Option::Some(12.1);
    
}