enum Color {
    _Green,
    Blue,
    _Red,
    _Black,
}

struct ShippingBox {
    height: f32,
    length: f32,
    breadth: f32,

    weight: f32,

    color: Color,
}

/* we needed to implement two functionality on the struct

1 -> creating a new ShippingBox instance
2 -> printing out its properties

 */

impl ShippingBox {
    fn create_shipping_box() -> Self {
        Self {
            height: 25.0,
            length: 25.0,
            breadth: 25.0,
            weight: 25.0,
            color: Color::Blue,
        }
    }

    fn new(height: f32, length: f32, breadth: f32, weight: f32, color: Color) -> Self {
        Self {
            height: height,
            length: length,
            breadth: breadth,
            weight: weight,
            color: color,
        }
    }

    fn print_properties(&self) {
        println!("{:?}", self.height);
        println!("{:?}", self.length);
        println!("{:?}", self.breadth);
        println!("{:?}", self.weight);

        match self.color {
            Color::_Black => println!("Black"),
            Color::_Green => println!("Green"),
            Color::Blue => println!("Blue"),
            Color::_Red => println!("Red"),
        }
    }
}

fn main() {

    // my old boring way
    let _box = ShippingBox::create_shipping_box();
    _box.print_properties();


    // my NEW WAYYY THIS IS SOOOO COOOL
    let _box_better = ShippingBox::new(
        23.2,
        34.5,
        23.4,
        543.333,
        Color::_Black,
    );
    _box_better.print_properties();

}
