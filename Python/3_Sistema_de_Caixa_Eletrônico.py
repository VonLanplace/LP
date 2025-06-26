import os


def clear_terminal():
    if os.name == "nt":
        os.system("cls")
    else:
        os.system("clear")


def carregar_notas():
    notas = []
    for i in range(0, 7):
        notas.append(100)
    return notas


def ler_notas(i):
    match i:
        case 0:
            nota = 2
        case 1:
            nota = 5
        case 2:
            nota = 10
        case 3:
            nota = 20
        case 4:
            nota = 50
        case 5:
            nota = 100
        case 6:
            nota = 200
        case _:
            nota = 0
    return nota


def valor_em_caixa(notas):
    soma = 0
    for i in range(0, len(notas)):
        nota = ler_notas(i)
        soma += notas[i] * nota
    return soma


def saque(valor, notas, i):
    nota = ler_notas(i)
    if nota > 0:
        if valor != nota + 1 and valor != nota + 3 and valor >= nota:
            notas[i] -= int(valor / nota)
        return saque(valor % nota, notas, i - 1)
    else:
        pass


def escreve_estatistica(estatisticas):
    with open("Resumo.txt", "w") as escrever:
        for i in range(0, len(estatisticas)):
            escrever.write(str(estatisticas[i]) + "\r\n")


def ler_estatistica():
    estatisticas = []
    with open("Resumo.txt", "r") as ler:
        while True:
            aux = ler.readline()
            if aux != "":
                estatisticas.append(aux)
            else:
                break
        return estatisticas


def retirar_notas(notas):
    i = 0
    estatisticas = []
    estatisticas.append(valor_em_caixa(notas))  # Total inicial
    while i < 5 and valor_em_caixa(notas) != 0:
        print("Valor em Caixa:", valor_em_caixa(notas))
        try:
            valor = int(input("Insira o valor a ser sacado: "))
        except Exception:
            valor = -1

        if valor > 1 and valor != 3 and valor <= valor_em_caixa(notas):
            saque(valor, notas, len(notas) - 1)
            clear_terminal()
            print("Sacou", valor, "R$")
            i += 1
        else:
            print("Valor inválido")

    total = estatisticas[0] - valor_em_caixa(notas)

    estatisticas.append(total / i)  # média dos saques
    estatisticas.append(total)  # Valor total dos saques
    estatisticas.append(i)  # Quantidade de Saques
    estatisticas.append(valor_em_caixa(notas))  # Total final
    escreve_estatistica(estatisticas)


def estatisticas():
    estatisticas = ler_estatistica()
    print(
        """
 |--------------------------------------
 |             Estatística             
 |--------------------------------------
""",
        "| 1 - Valor total inicial -",
        estatisticas[0],
        "| 2 - A média dos saques -",
        estatisticas[1],
        "| 3 - Valor total dos saques -",
        estatisticas[2],
        "| 4 - Quantidade de saques -",
        estatisticas[3],
        "| 5 - Valor total das sobras do caixa -",
        estatisticas[4],
        "|--------------------------------------",
    )
    input("Retornar (Enter).")

    clear_terminal()


clear_terminal()
notas = []
while True:
    print("""
|--------------------|
|   Menu Principal   |
|--------------------|
| 1 – Carregar Notas |
| 2 – Retirar Notas  |
| 3 – Estatística    |
| 9 – Fim            |
|--------------------|
""")
    try:
        opc = int(input("Opção: "))
    except Exception:
        opc = -1
    clear_terminal()
    match opc:
        case 1:
            notas = carregar_notas()
        case 2:
            if len(notas) > 0:
                retirar_notas(notas)
            else:
                print("Caixa não Carregado")
        case 3:
            estatisticas()
        case 9:
            clear_terminal()
            break
        case _:
            print("Opção Inválida!")
