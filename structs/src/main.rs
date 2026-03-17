struct BankAccount {
    owner : String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn TestBankAccount() {
    let mut account : BankAccount = BankAccount{owner: String::from("Alice"), balance: 1000.0};
    println!("{}'s initial balance: ${}", account.owner, account.get_balance());

    account.deposit(500.0);
    println!("{}'s balance after deposit: ${}", account.owner, account.get_balance());

    match account.withdraw(200.0) {
        Ok(_) => println!("{}'s balance after withdrawal: ${}", account.owner, account.get_balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    match account.withdraw(1500.0) {
        Ok(_) => println!("{}'s balance after withdrawal: ${}", account.owner, account.get_balance()),
        Err(e) => println!("Withdrawal failed: {}", e),
    }
}   

fn main() {
    TestBankAccount();
}
