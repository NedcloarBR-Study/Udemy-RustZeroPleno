fn main() {
  let notas = [6.0, 7.5, 9.0, 10.0];
  let media: f32 = calcular_media(&notas);
  println!("Média da nota: f{}", media);
}

fn calcular_media(notas: &[f32]) -> f32 {
  let mut soma: f32 = 0.0;
  for nota in notas {
    soma += *nota;
  }

  return soma / notas.len() as f32;
}
