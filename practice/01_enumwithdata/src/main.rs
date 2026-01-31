enum Product{
    Book { title: String, author: String, price: i32 },
    Electronics { name: String, brand: String, price: i32 },
    Clothing { item: String, size: String, price: i32 },
}

fn main(){
    let mut _book = Product::Book{
        title: String::from("The Rust Programming Language"),
        author: String::from("agus sulistiono"),
        price: 25000,
    };

    let mut _electronik = Product::Electronics{
        name: String::from("Smartphone"),
        brand: String::from("iPhone"),
        price: 3000000,
    };

    let mut _clothing = Product::Clothing{
        item: String::from("T-Shirt"),
        size: String::from("L"),
        price: 150000,
    };

    match _book{
        Product::Book{title, author, price}=>{
            println!("Buku: {}, Penulis: {}, Harga: {}", title, author, price);
        }
        Product::Electronics{name, brand, price}=>{
            println!("Elektronik: {}, Merek: {}, Harga: {}", name, brand, price);
        }
        Product::Clothing{item, size, price}=>{
            println!("Pakaian: {}, Ukuran: {}, Harga: {}", item, size, price);
        }
    }
}

