fn maior_numero(numero: Vec<i32>) -> i32 {
  let mut maior: i32 = numero[0];
  for i in 0..numero.len() {
    if maior < numero[i] {
      maior = numero[i];
    }
  }

  maior
}

fn main() {
  let vetor: Vec<i32> = vec![1, 2, 3, 4, 5];
  let maior: i32 = maior_numero(vetor);
  println!("Maior: {}", maior);
}
