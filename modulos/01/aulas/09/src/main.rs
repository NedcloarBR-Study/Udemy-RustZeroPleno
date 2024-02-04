fn dobro(num: i32) -> i32 {
  return num * 2;
}

fn maximo(a: i32, b: i32) -> i32 {
  if a > b {
    return a;
  } else {
    return b;
  }
}

fn alguma_fn(par_a: f32, par_b: i128) -> f32 {
  println!("Esta função devolve um valor flutuante");
  let x: f32 = 10.1f32 * par_a + par_b as f32;
  x
}

fn main() {
  println!("O dobro de 5 é: {}", dobro(5));
  println!("O maior entre de 5 e 4 é: {}", maximo(5, 4));
}
