use std::fs::{File, OpenOptions};
use std::io::prelude::*;

fn file_len(content: String) {
  let size = content.lines().count();
  println!("O arquivo tem {} linhas", size);
}

fn main() {
  let mut file = File::open("./src/dados.txt").expect("Erro ao abrir o arquivo.");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Erro ao ler o conteúdo do arquivo");

  println!("O conteúdo do arquivo é: \n\n {}", content);

  file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("./src/dados.txt")
    .expect("Erro ao abrir o arquivo com mais permissões");

  file
    .write_all(b"Conteudo extra")
    .expect("Erro ao escrever no arquivo");

  file_len(content);
}
