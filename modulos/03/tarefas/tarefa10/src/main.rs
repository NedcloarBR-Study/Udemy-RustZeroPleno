mod account;
use account::{Account, BankAccount, Currency};
fn main() {
  let mut account = Account::new(1000.0, Currency::USD);
  account.deposit(500.0);
  account.withdraw(500.0);
  println!("Account balance: {}", account.check_balance());
  let withdraw_result = account.withdraw(1200.0);
  if !withdraw_result {
    println!("Insufficient funds");
  } else {
    println!("Account balance: {}", account.check_balance());
  }
}
