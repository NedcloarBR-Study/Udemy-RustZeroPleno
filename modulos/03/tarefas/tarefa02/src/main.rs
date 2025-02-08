extern crate rand;
use rand::Rng;

fn gera_aleatorios() -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::new();
  for _ in 0..10 {
    let value = rand::thread_rng().gen_range(0, 101);
    vec.push(value);
  }
  return vec;
}

fn main() {
  let vec = gera_aleatorios();

  for i in vec {
    println!("{}", i);
  }
}
