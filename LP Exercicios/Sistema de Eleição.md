# TAREFA  –  MANIPULAÇÃO DE REGISTRO LÓGICO E ARQUIVO FÍSICO  

## **Em uma eleição há 2 candidatos em disputa**

- Criar um sistema que cadastre (votação) 5 eleitores  em cada grupo de votaçoes;
- Gravar também o cadastro eleitoral com 10 eleitores com seu número de eleitor e nome;
- Após os cadastramentos, processar e apurar:
  - vencedor,2º colocado, brancos, nulos, assim como suas quantidades apuradas.

|-|Votação 1 , 2 e Apuração|-|
|-|-|-|
|Seção|CodCandidato|Numero Eleitor|
|INT|INT|INT|

|   **CodCandidato**  |
|---------------------|
| 1 ----------- JOSE  |
| 2 ----------- MARIA |
| 3 ----------- BRANCO|
| 4 ----------- NULO  |

|ValidaSeção|
|-|
|[01 - 03 - 04 - 05 - 09 - 10]|
|Votação 1 > Seção 01,03,04|
|Votação 2 > Seção 05,09,10|

|-|Cadastro Eleitor|-|
|-|-|-|
|Numero Eleitor|Nome Eleitor|Seçao|
|INT|STRING|INT|

- Identificar o n. do eleitor e o nome , seção e qual o candidato votado;
- Após gerar os cadastros das votações 1 e 2, ordenar por CodCandidato esses arquivos em memória;
- Concatenar (Balance-Line) os 2 arquivos em um único com o nome apuração.
- Haverá um vetor (seção) com os nºs das seções válidas, caso cadastre (votação 1 e 2) errado, exiba mensagem " Seção inválida" e coletar nova seção;
- Haverá o arquivo Cadastro_Eleitor O NÚMERO DE ELEITOR, caso cadastre (VOTAÇÃO 1 E 2) errado, exibir mensagem "Número de eleitor inválido" e coletar novo número de eleitor para  valiação co campo Número de eleitor ;

|MENU PRINCIPAL|
|-|
|1 - Cadastra Eleitor|
|2 - Cadastra Votação 1 e 2|
|3 - Agrupa  Apuração|
|4 - Menu  Estatística|
|9 - FIM|

|Cadastro Votação 1,2|
|-|
|1 - Cadastro Votação 1|
|2 - Cadastro votação 2|
|9 - FIM|

|MENU ESTATÍSTICA|
|-|
| 1 - Vencedor|
| 2 - Segundo colocado|
| 3 - Quantidade de votos em branco|
| 4 - Quantidade de votos nulos|
| 5 - Mostra eleitores|
| 6 - Mostra apuração|
| 9 - FIM|
