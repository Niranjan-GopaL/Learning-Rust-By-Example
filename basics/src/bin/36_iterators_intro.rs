//! Introduction to Iterators
//! 

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]


pub trait  Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}




fn check_next_item_THE_ONLY_3_WAYS(){
    let v1 =vec![1,2,3,4];
    
    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(),Some(&1));    
    assert_eq!(v1_iter.next(),Some(&2));    
    assert_eq!(v1_iter.next(),Some(&3));    
    assert_eq!(v1_iter.next(),Some(&4));    
    assert_eq!(v1_iter.next(),None);    






    let mut  v2 =vec![1,2,3,4];

    let mut v2_iter = v2.iter_mut();
    
    assert_eq!(v2_iter.next(),Some(&mut 1));    
    assert_eq!(v2_iter.next(),Some(&mut 2));    
    assert_eq!(v2_iter.next(),Some(&mut 3));    
    assert_eq!(v2_iter.next(),Some(&mut 4));    
    assert_eq!(v2_iter.next(),None);    





    let v3 =vec![1,2,3,4];

    let mut v3_iter = v3.into_iter();
    
    assert_eq!(v3_iter.next(),Some(1));    
    assert_eq!(v3_iter.next(),Some(2));    
    assert_eq!(v3_iter.next(),Some(3));    
    assert_eq!(v3_iter.next(),Some(4));    
    assert_eq!(v3_iter.next(),None);    


}



fn the_2_types_of_ITERATOR_METHODES(){

    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();
    let result:u32 = v1_iter.sum();
    assert_eq!(result,15);

    // v1_iter.map(|x|x + 1);????

}


fn main()
{   
    let v1 =vec![1,2,3,4,5,6,7];
    let v1_iter = v1.iter();

    for value in v1_iter{
        println!("{}",value);
    }

    check_next_item_THE_ONLY_3_WAYS();
    the_2_types_of_ITERATOR_METHODES();
}