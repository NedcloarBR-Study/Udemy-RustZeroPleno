use std::{
  fs::File,
  io::{BufRead, BufReader, Error},
  process,
};

use tarefa03::exec;

fn ler_numeros(nome_arquivo: &str) -> Result<Vec<i32>, Error> {
  let arquivo = File::open(nome_arquivo)?;
  let leitor = BufReader::new(arquivo);
  let mut numeros = vec![];
  for linha in leitor.lines() {
    let linha = linha?;
    if let Ok(numero) = linha.trim().parse::<i32>() {
      numeros.push(numero);
    }
  }

  return Ok(numeros);
}

fn main() {
  let numeros = ler_numeros("./src/dados.txt").unwrap_or_else(|err| {
    eprintln!("Erro ao ler o arquivo: {}", err);
    process::exit(1);
  });
  exec(numeros);
}
