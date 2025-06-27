# Em um Banco haverá arquivos de Contas, Movimento e Atualizado

- Criar um sistema que cadastre contas de 5 clientes;
- Gravar também o cadastro de movimento com 10 registros com código conta,valor do movimento tipo de movimento e statusM;
- Após geração dos cadastros(contas e movimento), processar e gerar Contas Atualizadas, assim como suas consultas.

|-|-|ContasCorrentes|-|-|
|-|-|-|-|-|
|codContaC | nomeClientesC | saldoContaC | LimitecontaC |tipoContaC|
|int|String|double|double|int|

**tipoContaC**
   1 ----------- física
   2 ----------- conjunta
   3 ----------- jurídica
   4 ----------- especial

|-|MovimentoContas|-|-|
|-|-|-|-|
|codContaM | ValorMovimentoM | TipoMovimentoM |statusM|
|int|double|int|int|

**TipoMovimento**
   1 ----------- crédito
   2 ----------- débito

**StatusMovimento**
   1 ----------- Completo
   2 ----------- Em Aberto

- Classificar ambos os Arquivos em ordem crescente de CodConta
- Gerar CadastroAtualizado a partir do Balance-line dos arquivos    ContasCorrentes e MovimentoContas

|-|-|ContasAtualizados|-|-|
|-|-|-|-|-|
| codContaA | nomeClientesA | saldoContaA | LimitecontaA |tipoContaA|
| int|String|double|int|int|

1. Identificar a partir do codConta seu correspondente no codContaM, caso exista:
 1.1 se statusM = 1, gravar no ContasAtualizado, conforme lay-out;
 1.2 se statusM = 2, verificar TipoMovimentoM;
  1.2.1  caso saldoContaC < ValorMovimentoM

    |tipoContaC|Valor Limite|
    |-|-|
    |1 | zero|
    |2 | LimitecontaC |
    |3 | LimitecontaC + 50%  saldoContaC |
    |4 | LimitecontaC + 100% saldoContaC |

  1.2.2 caso saldoContaC > ValorMovimentoM
   calcular a subtração dos valores, se o resultado for negativo utilizar o critério do valor Limite

2. codConta < codContaM, gravar dados no  ContasAtualizados sem alteração;
  
|MENU PRINCIPAL|
|-|
| 1 - Cadastra Contas Correntes|
| 2 - Cadastra movimento|
| 3 - Gera Contas Atualizadas|
| 4 - Consulta cadastros|
| 9 - FIM|

|Consulta Cadastros|
|-|
| 1 - Contas Correntes|
| 2 - Movimento|
| 3 - Atualizado|
| 9 - FIM|
