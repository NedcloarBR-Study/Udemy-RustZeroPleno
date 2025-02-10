Instruções:

Implementação da Função:

1. Implemente a função mediana com a seguinte assinatura:

```Rust
use std::cmp::Ordering;

fn mediana(numeros: &Vec<i32>) -> Result<f64, &'static str>;
```

2. Entrega:

- Implemente o código conforme as instruções.
- Verifique se o código compila corretamente.
- Certifique-se de que a função mediana está calculando corretamente a mediana do vetor de números fornecido.

## Critérios de Avaliação

- A função mediana deve:
  - Retornar um erro com a mensagem "O vetor está vazio." caso o vetor fornecido esteja vazio.
  - Ordenar o vetor de entrada em ordem crescente.
  - Calcular corretamente a mediana, considerando vetores com número par e ímpar de elementos.
  - Utilizar a função auxiliar media (assinatura abaixo) para calcular a média quando necessário.
    `fn media(numeros: &Vec<f64>) -> f64;`
  - O código deve compilar sem erros.

A função mediana deve retornar um valor de ponto flutuante (`f64`) representando a mediana ou um erro (`&'static str`) se houver problemas.

## Observações

- Utilize os recursos da linguagem Rust para realizar a tarefa.
- Comente o código adequadamente para explicar o funcionamento de cada parte.
