mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut acc = BankAccount::new(100.0);

    println!("Initial balance: {}", acc.balance());

    acc.deposit(50.0);
    println!("After deposit: {}", acc.balance());

    acc.withdraw(30.0);
    println!("After withdrawal: {}", acc.balance());

    acc.withdraw(500.0); // ignored
    println!("After invalid withdrawal: {}", acc.balance());
}
