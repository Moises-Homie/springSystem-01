#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount >= 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount >= 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        let acc = BankAccount::new(100.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        let mut acc = BankAccount::new(50.0);
        acc.deposit(25.0);
        assert!((acc.balance() - 75.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit_negative() {
        let mut acc = BankAccount::new(50.0);
        acc.deposit(-10.0);
        assert!((acc.balance() - 50.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(40.0);
        assert!((acc.balance() - 60.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_too_much() {
        let mut acc = BankAccount::new(30.0);
        acc.withdraw(50.0);
        assert!((acc.balance() - 30.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_negative() {
        let mut acc = BankAccount::new(30.0);
        acc.withdraw(-10.0);
        assert!((acc.balance() - 30.0).abs() < EPSILON);
    }
}
