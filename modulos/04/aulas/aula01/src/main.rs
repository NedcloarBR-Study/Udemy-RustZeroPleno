use std::collections::HashMap;

fn media(numeros: &Vec<i32>) -> f64 {
  let mut soma = 0;
  for i in numeros {
    soma += i;
  }
  return soma as f64 / numeros.len() as f64;
}

fn mediana(numeros: &Vec<i32>) -> f64 {
  let mut numeros_sorted = numeros.clone();
  numeros_sorted.sort();
  let meio = numeros_sorted.len() / 2;
  if numeros_sorted.len() % 2 == 0 {
    let vec_meio = vec![numeros_sorted[meio], numeros_sorted[meio - 1]];
    return (media(&vec_meio));
  }
  return numeros_sorted[meio] as f64;
}

fn moda(numeros: &Vec<i32>) -> i32 {
  let mut map = HashMap::new();
  for i in numeros {
    let contar = map.entry(i).or_insert(0);
    *contar += 1;
  }

  let mut maior_valor = 0;
  let mut maior_key = 0;
  for (key, value) in map {
    if value > maior_valor {
      maior_valor = value;
      maior_key = *key;
    }
  }

  return maior_key;
}

fn main() {
  let numeros_media = vec![1, 1, 3, 3, 4, 4, 4, 5, 6, 0];
  println!("Média: {}", media(&numeros_media));
  let numeros_mediana = vec![4, 4, 6, 3, 1, 1, 4, 5, 2, 0];
  println!("Mediana: {}", mediana(&numeros_mediana));
  let numeros_moda = vec![0, 1, 2, 2, 3, 4];
  println!("Moda: {}", moda(&numeros_moda));
}
