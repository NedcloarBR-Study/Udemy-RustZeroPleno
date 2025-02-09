1. Crie uma estrutura de dados chamada "Operation" que contenha quatro variantes: "Addition", "Subtraction", "Multiplication" e "Division", cada um contendo dois campos do tipo "i32".

2. Escreva uma função chamada "calculate" que recebe uma instância de "Operation" como entrada e retorna um "Result" com um valor "i32" ou uma mensagem de erro "&'static str". A função deve usar um "match" para determinar qual operação está sendo solicitada e realizá-la. Se a operação for divisão e o divisor for zero, a função deve retornar "Err("Cannot divide by zero.")".

3. Teste sua função criando algumas instâncias de "Operation" e passando-as para a função "calculate". Certifique-se de tratar o caso de divisão por zero.

Dicas:

- Lembre-se de usar a cláusula "if" dentro do "match" para verificar se o divisor é zero antes de realizar a divisão.
- Use o método "unwrap()" com cuidado, pois ele pode causar panics se a operação retornar um erro. É melhor usar "match" para tratar casos de erro.
