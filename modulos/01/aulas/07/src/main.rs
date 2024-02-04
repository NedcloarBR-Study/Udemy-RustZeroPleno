use std::io;

fn convert_to_int(data_input: &String) -> i32 {
  let x: i32 = data_input.trim().parse::<i32>().unwrap();

  return x;
}

fn main() {
  let mut medias: String = String::new();
  io::stdin()
    .read_line(&mut medias)
    .expect("Erro ao ler medias");
  let mut soma_rec: i32 = 0;
  let mut i: i32 = 0;
  while convert_to_int(&medias) > i {
    let mut media_aluno: String = String::new();
    io::stdin()
      .read_line(&mut media_aluno)
      .expect("Erro ao ler media_aluno");
    i += 1;
    if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
      soma_rec += 1;
    }
  }

  println!("O número de alunos em recuperação é: {}", soma_rec);
}
