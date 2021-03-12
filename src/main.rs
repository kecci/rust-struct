fn main() {
    /* Initializing a Struct */
    let product_name = String::from("Algorithms");

    let order = OrderItem {
        product_name: product_name,
        price: 60.0,
        quantity: 1.0,
        user_id: 12851
    };
    println!("Order product_name {}", order.product_name);
    println!("Order price {}", order.price);
    println!("Order quantity {}", order.quantity);
    println!("Order user_id {}", order.user_id);

    // let mut order = OrderItem {
    //     product_name: product_name,
    //     price: 60.0,
    //     quantity: 1.0,
    //     user_id: 12851
    // };
    /* 
        You can't make fields mutable in a struct. 
        Rust doesn't allows to you do that. 
        Instead of that, make structs mutable.
    */

    /* Initializing Fields with Shorthand */
    let product_name = String::from("Algorithms");
    let price: f64 = 60.0;
    let quantity: f64 = 1.0;

    let order = OrderItem {
        product_name: product_name,
        price,
        quantity,
        user_id: 12851
    };
    println!("Order product_name {}", order.product_name);
    println!("Order price {}", order.price);
    println!("Order quantity {}", order.quantity);
    println!("Order user_id {}", order.user_id);

    /* The Struct Update Syntax */
    // let order2 = OrderItem {
    //     product_name: String::from("Rust by Example"),
    //     ..order
    // };
    // println!("Order detail {:?}", order2);

    // Example Usage of Structs
    let mut basket : Vec<&OrderItem> = Vec::new();
    let product_name = String::from("Rust by Example");
    let order_item = build_order_item(product_name, 45.0, 2.0, 19241);
    basket.push(&order_item);

    /* Tuple Structs */
    struct Color(i32, i32, i32);
    let rgb_color = Color(250, 128, 114); // Salmon
    println!("First color is {}", rgb_color.0);

    // Or we can destruct our struct;
    let Color(red, green, blue) = Color(250, 128, 114);
    println!("Red is {} Green is {} Blue is {}", red, green, blue);

    /* Unit Like Structs */
    #[derive(Debug)]
    struct MyType {
        the_field: u32
    }
    let instance = MyType { the_field: 5000 };
    println!("{:?}", instance);
    // https://stackoverflow.com/questions/41156082/in-rust-whats-the-difference-between-and-inside-a-println
}

struct OrderItem {
    // struct body
    product_name: String,
    price: f64,
    quantity: f64,
    user_id: u32
}

// Example Usage of Structs
fn build_order_item(product_name: String, price: f64, quantity: f64, user_id: u32) -> OrderItem {
    OrderItem {
        product_name,
        price,
        quantity,
        user_id
    }
}