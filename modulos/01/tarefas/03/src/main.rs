fn eh_primo(num: i32) -> bool {
  if num <= 1 {
    return false;
  }

  let limite: i32 = (num as f32).sqrt().ceil() as i32;

  for i in 2..=limite {
    if num % i == 0 {
      return false;
    }
  }

  return true;
}

fn main() {
  let num: i32 = 7;
  let resultado: bool = eh_primo(num);

  println!("O número {} é primo? {}", num, resultado);
}
