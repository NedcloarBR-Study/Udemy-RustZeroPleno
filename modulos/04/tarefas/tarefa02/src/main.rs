use std::{collections::HashMap, vec};

fn moda(numeros: &Vec<i32>) -> Option<i32> {
  let mut map = HashMap::new();
  for i in numeros {
    let contar = map.entry(i).or_insert(0);
    *contar += 1;
  }

  let mut maior_valor = 0;
  let mut maior_key = None;
  for (key, &value) in &map {
    if value > maior_valor {
      maior_valor = value;
      maior_key = Some(*key);
    }
  }

  let mut keys_empate = vec![];
  for (key, &value) in &map {
    if value == maior_valor {
      keys_empate.push(*key);
    }
  }

  if keys_empate.len() > 1 {
    return None;
  } else {
    return maior_key.copied();
  }
}

fn main() {
  let vec1 = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0]; // moda 4
  let vec2 = vec![1, 1, 2, 2, 3, 3, 4, 4]; // sem moda

  match moda(&vec1) {
    Some(v) => println!("Moda: {}", v),
    None => println!("Existem múltiplas modas."),
  }

  match moda(&vec2) {
    Some(v) => println!("Moda: {}", v),
    None => println!("Existem múltiplas modas."),
  }
}
