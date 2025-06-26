# MANIPULAÇÃO DE ARQUIVO FÍSICO (DESENVOLVER EM JAVA)

## CAIXA ELETRÔNICO (SAQUES E CONTROLES)

# 1. Criar um menu de opções

|Menu Principal|
|-|
|1 – Carregar Notas|
|2 – Retirar Notas|
|3 – Estatística|
|9 – Fim|

___

## 1.1. Carregar a quantidade: 100 cédulas de notas em uma área da memória com 7 ocorrências

## 1.2. Solicitar que o cliente faça a retirada de valores obedecendo o critério da maior para a menor nota

## 1.3. Caso não tenha o valor da maior cédula, disponibilizar a próxima nota maior

## 1.4. Solicitar até 100 retiradas ou não haver mais notas disponíveis no caixa

## 1.5. No final, exibir a estatística e gravar em arquivo: “Resumo.txt”, com

### 1.5.1.  Valor total inicial antes dos saques

### 1.5.2.  A média dos saques

### 1.5.3.  Valor total dos saques

### 1.5.4.  Quantidade de saques

### 1.5.5.  Valor total das sobras do caixa

### **Consistência do valor de entrada para o saque**

## 1.6. Após a solicitação do valor verificar

### 1.6.1 Valor não pode ser negativo e r$ 3,00

### 1.6.2 Menor valor possível é r$ 2,00 e maior será de r$ 3.000,00

### 1.6.3 Emitir mensagem “VALOR SOLICITADO INVÁLIDO”

## 1.7. Caso o valor solicitado não corresponda as notas a serem ofertadas, emitir mensagem “VALOR SOLICITADO NÃO PODE SER SACADO” e mostrar quais cédulas estão disponíveis

## 1.8. Se o valor a ser solicitado for maior que o saldo total do caixa, enviar a mensagem “EXCEDEU O LIMITE DO CAIXA”

## 1.9. Para todas as situações de consistência solicitar para que se possa fazer novo saque

___

|0|1|2|3|4|5|6|
|-|-|-|-|-|-|-|
|100|100|100|100|100|100|100|

|Referência de valor r$|2|5|10|20|50|100|200|
|-|-|-|-|-|-|-|-|

|-|
|-|
|Saque r$ 75,00  1 nota de 50,00, 1 nota de 20,00 e 1 nota de 5,00|
|Saque r$ 115,00  1 nota de 100,00, 1 nota de 10,00 e 1 nota de 5,00|
|Saque r$ 11,00  1 nota de 5,00 e 3 notas de 2,00|
|Saque r$ 53,00  2 notas de 20,00 e 1 nota de 5, 4 notas de 2,00|
