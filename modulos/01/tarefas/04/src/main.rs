use std::io;

fn convert_to_int(data_input: &String) -> i32 {
  let x: i32 = data_input.trim().parse::<i32>().unwrap();

  return x;
}

fn main() {
  let mut num: String = String::new();
  io::stdin().read_line(&mut num).expect("Erro ao ler num");
  let num: i32 = convert_to_int(&num);
  println!("Tabuada do {}:", num);
  for i in 1..11 {
    println!("{}x{} = {}", num, i, num * i);
  }
}
