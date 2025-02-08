Este é um programa em Rust que define uma estrutura chamada "Pessoa" com dois campos: "nome" do tipo "String" e "idade" do tipo "i32". Também define uma trait chamada "Voz" com dois métodos: "falar" e "tem_voz". A trait é então implementada para a estrutura "Pessoa". Na função principal, uma instância de "Pessoa" é criada e seu método "tem_voz" é chamado para verificar se a pessoa pode falar com base na idade dela. O programa imprime o resultado.

## Leitura  Complementar

Uma trait é como uma interface em outras linguagens de programação, ela define um conjunto de métodos que uma struct ou um tipo deve implementar. A trait "Voz" tem dois métodos: "falar" e "tem_voz". O método "falar" imprime uma saudação com o nome da pessoa, e o método "tem_voz" verifica se a pessoa tem idade suficiente para falar (maior do que 1 ano).

O código tem também uma implmentação da trait "Voz" para a struct "Pessoa", o que significa que a struct "Pessoa" implementa os métodos "falar" e "tem_voz" especificados na trait "Voz". Isso permite que as instâncias da struct "Pessoa" possam ser chamadas para falar e verificar se tem voz.

Na função principal, é criada uma instância da struct "Pessoa" com um nome e idade específicos, e então é chamado o método "tem_voz" para verificar se a pessoa pode falar, e imprime a saída "Pode [nome da pessoa] falar? [true or false]"

Você também pode implementar uma trait em tipos primitivos, como i32 ou f32, o que permite que esses tipos tenham comportamentos adicionais. Além disso, é possível definir traits genéricas que podem ser implementadas por vários tipos diferentes. Isso é feito usando "generic types" (tipos genéricos) na definição da trait, e especificando esses tipos genéricos quando implementando a trait. Traits são uma ferramenta poderosa em Rust que permitem a criação de código mais genérico e reutilizável. Eles também permitem a criação de tipos polimórficos, o que é essencial para a programação orientada a objetos em Rust.
