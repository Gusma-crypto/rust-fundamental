//this main concep is struct in enum;

enum GameAction {
    Move { x: i32, y: i32 },
    Attack { target: String, damage: u32 },
    UseItem { item: String },
    Say { message: String },
}

fn main() {
    let _action1 = GameAction::Move { x: 10, y: 20 };
    let _action2 = GameAction::Attack {
        target: String::from("Goblin"),
        damage: 50,
    };
    let _action3 = GameAction::UseItem {
        item: String::from("Potion"),
    };
    let _action4 = GameAction::Say {
        message: String::from("I am ready for battle!"),
    };

    // Process action
    //change action4 to see different outputs action1 action2 action3
    match _action4 {
        GameAction::Move { x, y } => {
            println!("Bergerak ke posisi ({}, {})", x, y);
        }
        GameAction::Attack { target, damage } => {
            println!("Menyerang {} dengan damage {}!", target, damage);
        }
        GameAction::UseItem { item } => {
            println!("Menggunakan {}", item);
        }
        GameAction::Say { message } => {
            println!("Berkata: {}", message);
        }
    }
}