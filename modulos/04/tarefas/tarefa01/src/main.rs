fn mediana(numeros: &Vec<i32>) -> Result<f64, &'static str> {
  if numeros.len() == 0 {
    return Err("Não existem elementos no vetor");
  }

  let mut numeros_sorted = numeros.clone();
  numeros_sorted.sort();
  let meio = numeros_sorted.len() / 2;
  if numeros_sorted.len() % 2 == 0 {
    let vec_meio = vec![numeros_sorted[meio], numeros_sorted[meio - 1]];
    return Ok(media(&vec_meio));
  }
  return Ok(numeros_sorted[meio] as f64);
}

fn media(numeros: &Vec<i32>) -> f64 {
  let mut soma = 0;
  for i in numeros {
    soma += i;
  }
  return soma as f64 / numeros.len() as f64;
}

fn main() {
  let numeros = vec![1, 1, 2, 3, 3, 4, 5];
  match mediana(&numeros) {
    Ok(mediana) => println!("Mediana: {}", mediana),
    Err(mensagem) => println!("Erro: {}", mensagem),
  }
}
