/*   WHY THIS WAY ???

    -  Having all the logic in one place => EXTREMELY BAD
    -  So BREAK IN TO SMALL PIECES => EACH PIECES WITH IT OWN FUNCTIONALITY.

    - All information about color is in ONE SINGLE PLACE
    - All information about dimensions is in ONE SINGLE PLACE.

$$$$    So we can make changes in that single places without effecting any  $$$$ 
$$$$    other part of the code. THIS IS A REALLY IMPORTANT PHILOSOPHY       $$$$            

 */



// <-------------------------------------- COLOR ENUM IS CREATED HERE --------------------------------
enum Color {
    _Green,
    _Blue,
    _Red,
    _Black,
}

impl Color{
    fn print(&self) {
        match self  {
            Color::_Black => println!("Black"),
            Color::_Red => println!("Red"),
            Color::_Blue => println!("_Blue"),
            Color::_Green => println!("Green")
        }
    }
}
// <----------------------------------------------------------------------------------------------------



// <-------------------------------------- DIMENSION STRUCT IS CREATED HERE --------------------------------
struct Dimension{
    _height: f32,
    _length: f32,
    _breadth: f32,
}


impl Dimension{
    fn print(&self){
        println!("_height: {}", self._height);
        println!("_length: {}", self._length);
        println!("_breadth: {}", self._breadth);
    }
}
// <----------------------------------------------------------------------------------------------------




// <-------------------------------------- SHIPPING BOX IS CREATED HERE --------------------------------
struct ShippingBox{
    _dimensions: Dimension,
    _color : Color,
    _weight  : f32,
}

impl ShippingBox{
    fn new(_dimensions: Dimension, _color: Color, _weight: f32) -> Self {
        Self { _dimensions, _color, _weight }
    }
    
    fn print(&self){
        self._dimensions.print();
        self._color.print();
        println!("{:?}",self._weight);
        println!()
    }
}
// <----------------------------------------------------------------------------------------------------





fn main() {

    // Lets create a small box
    let small_dimensions = Dimension{
        _height: 1.2,
        _length: 0.3,
        _breadth:5.4};
    let small_color = Color::_Green;
    let small_weight = 23.4;

    let small_box = ShippingBox::new(small_dimensions, small_color, small_weight);
    small_box.print();
    
    
    // Let's create a large box
    let large_dimensions = Dimension{
        _height: 110000.231,
        _length: 1214312.32,
        _breadth:4123412.123};
    let large_color = Color::_Black;
    let large_weight = 23.4;

    let large_box = ShippingBox::new(large_dimensions, large_color, large_weight);
    large_box.print();

}