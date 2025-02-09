pub trait BankAccount {
  fn new(balance: f64, currency: Currency) -> Self;
  fn deposit(&mut self, amount: f64);
  fn withdraw(&mut self, amount: f64) -> bool;
  fn check_balance(&self) -> f64;
}

pub enum Currency {
  USD,
  EUR,
  GBP,
}

pub struct Account {
  pub balance: f64,
  pub currency: Currency,
}

impl BankAccount for Account {
  fn new(balance: f64, currency: Currency) -> Self {
    Account { balance, currency }
  }

  fn deposit(&mut self, amount: f64) {
    self.balance += amount;
  }

  fn withdraw(&mut self, amount: f64) -> bool {
    if self.balance >= amount {
      self.balance -= amount;
      true
    } else {
      false
    }
  }

  fn check_balance(&self) -> f64 {
    self.balance
  }
}
