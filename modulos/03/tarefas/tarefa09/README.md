1 - Crie uma nova pasta para o seu projeto e inicie um novo arquivo chamado "finance.rs".

2 - Defina um módulo chamado "finance", que contém uma struct chamada "Asset", com as seguintes características:

*O nome do ativo.

*O valor do ativo.

*A categoria do ativo (ações, títulos, fundos, etc.).

3 - Defina outra struct chamada "Portfolio", que contém as seguintes características:

*Um vetor de ativos

*Uma função "new()" que retorna uma nova instância vazia de Portfolio.

4 - Implemente uma função "add_asset()" para a struct "Portfolio" que adiciona um ativo ao portfólio.

5 - Implemente uma função "total_value()" para a struct "Portfolio" que calcula o valor total do portfólio, somando o valor de todos os ativos contidos no vetor.

6 - No arquivo main.rs, crie uma nova instância de Portfolio e adicione alguns ativos a ele, usando a função "add_asset()".

7 - Utilize a função "total_value()" para calcular o valor total do portfólio e imprima o resultado.

8 - Execute o código usando o comando "cargo run" na linha de comando e verifique o resultado.
