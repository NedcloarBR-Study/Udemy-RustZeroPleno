pub enum Category {
  Stock,
  Bond,
  RealEstate,
  Commodity,
  Crypto,
  Cash,
}

pub struct Asset {
  pub name: String,
  pub price: f64,
  pub category: Category,
}

pub struct Portfolio {
  pub assets: Vec<Asset>,
}

impl Portfolio {
  pub fn new() -> Portfolio {
    Portfolio { assets: Vec::new() }
  }

  pub fn add_asset(&mut self, asset: Asset) {
    self.assets.push(asset);
  }

  pub fn total_value(&self) -> f64 {
    self.assets.iter().map(|asset| asset.price).sum()
  }
}
