enum HeroStatus {
    Idle,       // 0
    Fighting,   // 1
    Resting,    // 2
    Dead,       // 3
}

struct Hero {
    name: String,
    health: u32,
    status: HeroStatus,
}

fn main() {
    let mut hero = Hero {
        name: String::from("Budi"),
        health: 100,
        status: HeroStatus::Idle,
    };

    // Ubah status
    hero.status = HeroStatus::Fighting;

    // Cek status dengan match
    match hero.status {
        HeroStatus::Idle => println!("{} sedang idle", hero.name),
        HeroStatus::Fighting => println!("{} sedang bertarung!", hero.name),
        HeroStatus::Resting => println!("{} sedang istirahat", hero.name),
        HeroStatus::Dead => println!("{} mati!", hero.name),
    }
}