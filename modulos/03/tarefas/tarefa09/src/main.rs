mod finance;
use finance::{Asset, Category, Portfolio};

fn main() {
  let mut portfolio = Portfolio::new();
  let asset = Asset {
    name: "Bitcoin".to_string(),
    price: 10000.0,
    category: Category::Crypto,
  };
  portfolio.add_asset(asset);
  print!("Total value: {}", portfolio.total_value());
}
