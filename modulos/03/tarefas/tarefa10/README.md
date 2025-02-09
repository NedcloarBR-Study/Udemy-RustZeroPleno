1. Crie uma nova pasta para o seu projeto e inicie um novo arquivo chamado "account.rs".

2. Defina um trait chamado "BankAccount" que contém as seguintes funções:

- "new(balance: f64, currency: Currency) -> Self", que cria uma nova instância de conta bancária com o saldo e a moeda passados como parâmetro.

- "deposit(&mut self, amount: f64)", que permite depositar dinheiro na conta.

- "withdraw(&mut self, amount: f64) -> bool", que permite sacar dinheiro da conta, retornando "true" se o saque foi realizado com sucesso e "false" se o saldo for insuficiente.

- "check_balance(&self) -> f64", que retorna o saldo atual da conta.

3. Defina um enum chamado "Currency", que contém as opções de moedas disponíveis para a conta (USD, EUR, GBP, etc).

4. Defina uma struct chamada "Account" que implementa o trait "BankAccount" e contém as seguintes características:

- Um campo "balance" para armazenar o saldo da conta.

- Um campo "currency" para armazenar a moeda da conta.

5. Vá para o arquivo main.rs e na função "main", crie uma nova instância de "Account" passando o saldo inicial (1000.0) e a moeda (Currency::USD).

6. Utilize o método "deposit" para depositar 500.0 na conta e imprima o novo saldo.

7. Utilize o método "withdraw" para tentar sacar 1200.0 da conta e verifique se foi possível (usando a variável "withdrawal_success"). Se sim, imprima o novo saldo. Se não, imprima uma mensagem de erro.
