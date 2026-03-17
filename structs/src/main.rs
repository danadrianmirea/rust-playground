#![allow(dead_code)]
#![allow(unused_variables)]

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

fn test_bank_account() {
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

// composition example: a SavingsAccount is composed of a BankAccount and has an additional interest_rate field
struct SavingsAccount {
    account: BankAccount,
    interest_rate: f64,
}

impl SavingsAccount {
    fn new(owner: String, balance: f64, interest_rate: f64) -> SavingsAccount {
        SavingsAccount {
            account: BankAccount { owner, balance },
            interest_rate,
        }
    }

    fn apply_interest(&mut self) {
        let interest = self.account.get_balance() * self.interest_rate;
        self.account.deposit(interest);
    }
}


fn test_savings_account() {
    let mut savings = SavingsAccount::new(String::from("Bob"), 2000.0, 0.05);
    println!("{}'s initial balance: ${}", savings.account.owner, savings.account.get_balance());

    savings.apply_interest();
    println!("{}'s balance after applying interest: ${}", savings.account.owner, savings.account.get_balance());
}

use std::any::Any;

trait Animal {
    fn make_sound(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Dog {
    name: String,
    breed: String
}

impl Dog {
    fn new(name: String, breed: String) -> Dog {
        Dog { name, breed }
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Cat {
    name: String,
    color: String
}

impl Cat {
    fn new(name: String, color: String) -> Cat {
        Cat { name, color }
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Lion {
    cat: Cat,
    mane_size: f64
}

impl Animal for Lion {
    fn make_sound(&self) {
        println!("Roar!");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn test_polimorphism() {
    let dog = Dog::new(String::from("Buddy"), String::from("Golden Retriever"));
    let cat = Cat::new(String::from("Whiskers"), String::from("Tabby"));
    let lion: Lion = Lion { cat: Cat::new(String::from("Simba"), String::from("Golden")), mane_size: 1.5 };

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat), Box::new(lion)];

    for animal in animals {
        animal.make_sound();
        // if animal is a lion, we can access its mane_size through downcasting
        if let Some(lion) = animal.as_any().downcast_ref::<Lion>() {
            println!("This lion has a mane size of {} meters.", lion.mane_size);
        }
    }
}


fn main() {
    test_bank_account();
    test_savings_account();
    test_polimorphism();
}
