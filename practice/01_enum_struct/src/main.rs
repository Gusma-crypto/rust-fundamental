struct Wallet {
    balance: u64,
}
enum Action {
    Deposit(u64),
    Withdraw(u64),
}
impl Wallet {
    fn apply(&mut self, action: Action) {
        //match action seperti switch case
        //atau if else berlapis
        //jik action dipilin deposit maka lakukan aksi deposit
        //jika action di pilih witdhtraw maka lakukan aksi withdraw
        match action {
            //jika action deposit
            Action::Deposit(amount) => {
                self.balance += amount;
                println!("Deposit {} → Saldo {}", amount, self.balance);
            }
            //jika action withdraw
            Action::Withdraw(amount) => {
                if self.balance >= amount {
                    self.balance -= amount;
                    println!("Withdraw {} → Saldo {}", amount, self.balance);
                } else {
                    println!("Saldo tidak cukup");
                }
            }
        }
    }
}

fn main() {
    let mut wallet = Wallet { balance: 100 };

    wallet.apply(Action::Deposit(50));
    wallet.apply(Action::Withdraw(30));
    wallet.apply(Action::Withdraw(20));
}