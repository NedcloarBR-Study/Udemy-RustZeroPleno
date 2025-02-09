pub trait Shape {
  fn area(&self) -> f64;
  fn perimeter(&self) -> f64;
  fn draw(&self);
}

pub struct Circle {
  pub radius: f64,
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    return std::f64::consts::PI * self.radius * self.radius;
  }

  fn perimeter(&self) -> f64 {
    return 2.0 * std::f64::consts::PI * self.radius;
  }

  fn draw(&self) {
    println!("Desenhando um círculo com raio: {}", self.radius);
  }
}
