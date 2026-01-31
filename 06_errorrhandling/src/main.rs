struct Product {
    name: String,
    price: i32,
}

impl Product {
    fn new(name: &str, price: i32) -> Result<Product, String> {
        if price < 0 {
            return Err(String::from("Price cannot be negative"));
        }
        Ok(Product {
            name: name.to_string(),
            price,
        })
    }

    // fn new(name: &str, price: i32) -> Result<Product, String> {
    //     if price < 0 {
    //         return Err(String::from("Price cannot be negative"));
    //     }

    //     Ok(Product {
    //         name: name.to_string(),
    //         price,
    //     })
    // }
    
}

fn main(){
    match Product::new("Laptop", 1500) {
        Ok(product) => println!("Product created: {} with price {}", product.name, product.price),
        Err(e) => println!("Error creating product: {}", e),
    }

    match Product::new("Smartphone", -500) {
        Ok(product) => println!("Product created: {} with price {}", product.name, product.price),
        Err(e) => println!("Error creating product: {}", e), // "Price cannot be negative"
    }
}