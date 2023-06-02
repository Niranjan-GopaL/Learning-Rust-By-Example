fn expression__() {
    // Example of how nested expression works

    enum Order {
        _Drink,
        _Mac,
        _Burger,
        _Pizza,
        _Tacobell,
    }

    let item_ordered = Order::_Drink; 
    let drink_type_ordered = "water";
    let _paid = true;
    let _order_placed_from_drink_section = match item_ordered {
        // checkes is item_ordered is a _Drink variant
        Order::_Drink => {
            if drink_type_ordered == "water" {
                true
            } else {
                false
            }
        },
        // for anything else order need not be placed from drink section
        _ => false,
    };

    print!("{}",_order_placed_from_drink_section);

    /*  Notice this important technique

    Order::_Drink =>     {                                            <-----
                            if drink_type_ordered == "water" {            |
                                true                                      \
                            } else {                                      /
                                false                                    |   
                            }                                            \   
                        },                                          <-----
                                                                         |
                                                                         |
                                                            THIS IS AN EXPRESSION 

    WE WANTED TO DO SOEMTHING WHEN THEY MATCHED =====>

    ANSWER :- JUST DO A 
    {
        DO WHATEVER YOU WANT TO DO
    }
    
     */



}

fn main() {
    expression__();
}
