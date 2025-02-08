use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut arquivo = File::create("teste.txt").expect("Erro ao criar o arquivo");
  arquivo
    .write_all(b"Arquivo de teste sendo criado")
    .expect("Erro ao escrever no aquivo");
}
