enum Color{
    Green,
    Blue,
    Red,
    Black
}

struct ShippingBox{
    height:f32,
    length:f32,
    breadth:f32,

    weight:f32,

    color:Color,
}


impl ShippingBox{
    fn create_shipping_box() -> Self {
        Self { height: 25.0, length: 25.0, breadth: 25.0, weight: 25.0, color:Color::Blue  }
    }

    fn print_properties(&self) {

        println!("{:?}", self.height);
        println!("{:?}", self.length);
        println!("{:?}", self.breadth);
        println!("{:?}", self.weight);
       
       match self.color {

        Color::Black => println!("Black"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),

       }

    }

}


fn main() {
    let _box = ShippingBox::create_shipping_box();
    _box.print_properties();
    
}