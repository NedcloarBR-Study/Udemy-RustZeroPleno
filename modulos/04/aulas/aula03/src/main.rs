use std::{collections::HashMap, io::stdin};

fn departamento() {
  let mut departamento_pessoas = HashMap::new();

  loop {
    println!("Digite o mando do tipo: Adicione <Pessoa> para <Departamento>");
    let mut comando = String::new();
    stdin()
      .read_line(&mut comando)
      .expect("Erro ao ler variavel comando");
    let comando = comando.trim();
    let mut token_comando = comando.split_whitespace();
    let pessoa = match token_comando.nth(1) {
      Some(pessoa) => pessoa,
      None => {
        println!("Erro ao obter o nome da pessoa");
        continue;
      }
    };
    let departamento = match token_comando.nth(1) {
      Some(departamento) => departamento,
      None => {
        println!("Erro ao obter o nome do departamento");
        continue;
      }
    };

    let empregado = departamento_pessoas
      .entry(String::from(departamento))
      .or_insert(vec![]);
    empregado.push(String::from(pessoa));
    println!("{:?}", departamento_pessoas);
  }
}

fn main() {
  departamento();
}
