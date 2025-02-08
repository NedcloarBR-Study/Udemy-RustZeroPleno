extern crate rand;
use rand::Rng;

fn main() {
  let valores_random = rand::thread_rng().gen_range(5., 11.);
  println!("{}", valores_random);
}
