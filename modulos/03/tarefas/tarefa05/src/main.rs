mod shapes;
use shapes::{Circle, Shape};

fn main() {
  let circle = Circle { radius: 5. };

  println!("Área: {}", circle.area());
  println!("Perímetro: {}", circle.perimeter());
  circle.draw();
}
