fn main() {
    struct Product {
        name: String,
        price: f32,
        color: String,
        quantity: i32,
    }

    impl Product {
        fn new(name: String, price: f32, color: String, quantity: i32) -> Product {
            Product {
                name,
                price,
                color,
                quantity,
            }
        }

        fn calculate_total_price(&self) -> f32 {
            self.price * self.quantity as f32
        }
    }

    let inputs = input_product();
    let product = Product::new(inputs.0, inputs.1, inputs.2, inputs.3);
    println!("Total price of {} is {}", product.name, product.calculate_total_price());
}

fn input_product() -> (String, f32, String, i32) {
    let mut product = String::new();
    let mut price = String::new();
    let mut color = String::new();
    let mut quantity = String::new();

    println!("Enter product name: ");
    std::io::stdin().read_line(&mut product).expect("Failed to read line");

    println!("Enter product price: ");
    std::io::stdin().read_line(&mut price).expect("Failed to read line");

    println!("Enter product color: ");
    std::io::stdin().read_line(&mut color).expect("Failed to read line");

    println!("Enter product quantity: ");
    std::io::stdin().read_line(&mut quantity).expect("Failed to read line");

    let product = product.trim().to_string();
    let price = price.trim().parse::<f32>().expect("Failed to parse price");
    let color = color.trim().to_string();
    let quantity = quantity.trim().parse::<i32>().expect("Failed to parse quantity");

    return (product, price, color, quantity)
}
