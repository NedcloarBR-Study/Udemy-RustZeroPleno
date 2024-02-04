1 - Qual das opções abaixo é uma característica das funções em Rust?
[] Podem ser chamadas várias vezes no código.
[] Podem retornar um valor.
[] Podem receber parâmetros.
[X] Todas as opções acima.

2- Qual das opções abaixo é uma estrutura de fluxo de controle em Rust?
[] `for`.
[] `while`.
[] `match`.
[X] Todas as opções acima.

3 - Qual das opções abaixo não é um tipo de dado em Rust?
[] `i32`.
[X] `char32`.
[] `f64`.
[] `bool`.

4 - Qual das opções abaixo é um operador de comparação em Rust?
[] +.
[] -.
[ ] ==. <-- Correto
[X] &&. <-- Incorreto

5 - O que é uma closure em Rust?
[X] Uma função anônima que pode ser armazenada em uma variável ou passada como argumento para outra função. <-- Incorreto
[] Uma função que é chamada automaticamente quando um evento ocorre.
[] Uma função que pode acessar e modificar as variáveis do escopo em que ela foi definida.
[] Todas as Opções acima. <-- Correto

6 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  let x = 3;
  let y = 4;
  let z = if x < y { x } else { y };
  println!("{}", z);
}
```

[X] 3.
[] 4.
[] 7.
[] Nenhuma.

7 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  for i in 1..4 {
    println!("{}", i);
  }
}
```

[X] 1 2 3.
[] 2 3 4.
[] 1 2 3 4.
[] Nenhuma das opções acima.

8 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  let x = true;
  let y = false;
  let z = x || y;
  println!("{}", z);
}
```

[X] `true`.
[] `false`.
[] 1.
[] 0.

9 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  let x = 5;
  let y = 10;
  let z = x + y;
  println!("{}", z);
}
```

[] 5.
[] 10.
[X] 15.
[] Nenhuma das opções acima.

10 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  let x = 5;
  let y = 10;
  let z =
  if x < y {
    if x % 2 == 0 {
      x \* 2
    } else {
      y - x
    }
  } else {
    if y % 2 == 0 {
      y / 2
    } else {
      x + y
    }
  };
  println!("{}", z);
}
```

[] 5. <-- Correto
[] 10.
[] 15.
[X] Nenhuma das opções acima. <-- Incorreto

11 - Qual é a saída do seguinte código em Rust?

```Rust
fn main() {
  let x = 10;
  let y = 15;
  let z =
  if x < y {
    if x % 3 == 0 {
      x \* 2
    } else {
      y - x
    }
  } else {
    if y % 3 == 0 {
      y / 2
    } else {
      x + y
    }
  };
  println!("{}", z);
}
```

[] 10
[] 15
[] 25
[X] Nenhuma das opções acima.

12 - Qual é a finalidade principal da linguagem de programação Rust?
[] Criar aplicativos de desktop
[] Desenvolvimento de jogos
[X] Segurança e estabilidade de sistemas operacionais
[] Desenvolvimento de aplicativos móveis

13 - Como Rust garante a segurança de memória?
[] Utilizando alocação dinâmica de memória
[] Implementando verificações de tipos estáticos
[] Utilizando ponteiros seguros
[X] Todas as alternativas acima.

Resultado final:
Tentativa 1: Aprovado! (é necessário ter uma pontuação de 50% para ser aprovado)
76%
de acertos
(10/13)
13 minutos
4 de fevereiro de 2024 às 00:09

Nota: Fiz com pressa 🤦🏼‍♂️ e acabei errado 3 questões. Provavelmente esse Readme está uma bagunça outra hora arrumo estou com sono kkkkk.
