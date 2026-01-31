//EROR HANDLING WITH RESUT<T, E>
/*
=======Result, Ok(item), Err(error) =======

Result<T, E> - bisa Ok(value) atau Err(error)
Oke(T)- ada nilai yang diharapkan
Err(E) - ada error yang terjadi
Ok(item) - item ditemukan
Err(error) - item tidak ditemukan
*/  
//start code here

#[allow(dead_code)]
struct Hero {
    name: String,
    health: u32,
    mana: u32,
}
#[allow(dead_code)]
impl Hero {
    fn cast_spell(&mut self, cost: u32) -> Result<(), String> {
        if self.mana < cost {
            return Err(String::from("Mana tidak cukup!"));
        }

        self.mana -= cost;
        Ok(())
    }
    
    fn take_damage(&mut self, damage: u32) -> Result<(), String> {
        if damage >= self.health {
            self.health = 0;
            return Err(String::from("Hero mati!"));
        }

        self.health -= damage;
        Ok(())
    }
}

fn main() {
    
    let mut hero = Hero {
        name: String::from("Budi"),
        health: 100,
        mana: 30,
    };

    // Coba cast spell (berhasil)
    match hero.cast_spell(20) {
        Ok(()) => println!("Spell berhasil dicast!"),
        Err(e) => println!("Error: {}", e),
    }

    // Coba cast lagi (mana tidak cukup)
    match hero.cast_spell(20) {
        Ok(()) => println!("Spell berhasil dicast!"),
        Err(e) => println!("Error: {}", e),  // "Mana tidak cukup!"
    }
}



//END code here



/*
ERORR HANDLING WITH OPTION<T>
=======Option, Some(item), None/null =======
Option<T> - bisa Some(value) atau None
T- ada nilai yang diharapkan(String Integer dll)
Some(item) - item ditemukan
None - item tidak ditemukan
Seperti null tapi lebih aman!
*/

//start code here
/*
struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    fn get_item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    fn add_item(&mut self, item: String) {
        self.items.push(item);
    }
}

fn main() {
    let mut inventory = Inventory {
        items: vec![
            String::from("Sword"),
            String::from("Shield"),
            String::from("Potion"),
        ],
    };
    //mencoba menam,bah item baru
    inventory.add_item(String::from("Helmet"));
    inventory.add_item(String::from("Boots"));


    // Coba ambil item yang ada
    match inventory.get_item(0) {
        Some(item) => println!("Ditemukan item: {}", item),
        None => println!("Item tidak ditemukan!"),
    }

    // Coba ambil item yang tidak ada
    match inventory.get_item(10) {
        Some(item) => println!("Ditemukan: {}", item),
        None => println!("Item tidak ditemukan!"),  // Ini yang akan jalan
    }

    //coba ambil item yang baru ditambahkan
    match inventory.get_item(4) {
        Some(item) => println!("Ditemukan item baru ditambahkan: {}", item),
        None => println!("Item tidak ditemukan!"),
    }
}
*/
//end code here