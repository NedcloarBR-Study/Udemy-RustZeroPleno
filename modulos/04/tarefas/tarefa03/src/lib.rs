use std::collections::HashMap;

pub struct EstatisticasDescritivas {
  numeros: Vec<i32>,
}

impl EstatisticasDescritivas {
  pub fn new(numeros: Vec<i32>) -> Self {
    return EstatisticasDescritivas { numeros };
  }

  pub fn media(numeros: &Vec<f64>) -> f64 {
    let soma: f64 = numeros.iter().sum();
    return soma / numeros.len() as f64;
  }

  pub fn mediana(&self) -> Result<f64, &'static str> {
    let mut numeros_sorted = self.numeros.clone();
    numeros_sorted.sort();
    if numeros_sorted.len() == 0 {
      return Err("Lista vazia");
    }
    let meio = numeros_sorted.len() / 2;
    if numeros_sorted.len() % 2 == 0 {
      let vec_meio = vec![numeros_sorted[meio] as f64, numeros_sorted[meio - 1] as f64];
      return Ok(EstatisticasDescritivas::media(&vec_meio));
    }
    return Ok(numeros_sorted[meio] as f64);
  }

  pub fn moda(&self) -> Vec<i32> {
    let mut map = HashMap::new();
    for &numero in self.numeros.iter() {
      *map.entry(numero).or_insert(0) += 1;
    }

    let max = map.values().cloned().max().unwrap_or(0);

    return map
      .into_iter()
      .filter(|&(_, v)| v == max)
      .map(|(k, _)| k)
      .collect();
  }
}

pub fn exec(numeros: Vec<i32>) {
  let estatisticas = EstatisticasDescritivas::new(numeros.clone());

  match estatisticas.mediana() {
    Ok(mediana) => println!("Mediana: {:.2}", mediana),
    Err(err) => println!("Erro ao calcular mediana: {}", err),
  };

  let numeros_f64: Vec<f64> = numeros.iter().map(|&x| x as f64).collect();
  let media_resultado = EstatisticasDescritivas::media(&numeros_f64);
  println!("Média: {:.2}", media_resultado);

  let moda_resultado = estatisticas.moda();
  println!("Moda: {:?}", moda_resultado);
}
