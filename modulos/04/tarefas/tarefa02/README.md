Implementação da Função moda

Implemente a função moda que recebe um vetor de números inteiros (&[i32]) e retorna a moda como Option<i32>.

Utilize um HashMap para contar as ocorrências de cada número no vetor.

Encontre a chave com a maior contagem para determinar a moda.

Caso haja empate na contagem (múltiplas modas), retorne None.

Impressão da Moda

No final da função main, imprima a moda calculada ou indique se existem múltiplas modas.

Exemplo de Entrada e Saída

Use o vetor de números vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0] como exemplo de entrada.

A saída esperada para esse vetor é Moda: 4, pois o número 4 é o mais frequente.

Se houver empate na contagem (por exemplo, vec![1, 1, 2, 2, 3, 3, 4, 4]), a saída deve ser Existem múltiplas modas..
