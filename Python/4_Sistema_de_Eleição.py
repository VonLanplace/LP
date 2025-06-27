import os


def clear_terminal():
    if os.name == "nt":
        os.system("cls")
    else:
        os.system("clear")


class eleitor:
    # |Numero Eleitor|Nome Eleitor|Seçao|
    def __init__(self) -> None:
        self.new(0, "", 0)
        pass

    def new(self, numero_eleitor, nome_eleitor, secao):
        self.numero_eleitor = numero_eleitor
        self.nome_eleitor = nome_eleitor
        self.secao = secao


class voto:
    # |Seção|CodCandidato|Numero Eleitor|
    def __init__(self) -> None:
        self.new(0, 0, 0)
        pass

    def new(self, secao, codigo_candidadto, numero_eleitor):
        self.secao = secao
        self.codigo_candidadto = codigo_candidadto
        self.numero_eleitor = numero_eleitor


def escrever_eleitor(eleitores):
    with open("Eleitores.md", "w") as escrever:
        escrever.write("|Numero Eleitor|Nome Eleitor|Seçao|\r\n")
        escrever.write("|-|-|-|\r\n")
        for eleitor in eleitores:
            escrever.write(
                "|"
                + str(eleitor.numero_eleitor)
                + "|"
                + eleitor.nome_eleitor
                + "|"
                + str(eleitor.secao)
                + "|\r\n"
            )


def ler_eleitor():
    eleitores = []
    with open("Eleitores.md", "r") as ler:
        # Skiping Headers
        ler.readline()
        ler.readline()
        # Reading line 1
        linha = ler.readline().split("|")
        # Loop de leitura
        while len(linha) > 1:
            pessoa = eleitor()
            pessoa.numero_eleitor = int(linha[1])
            pessoa.nome_eleitor = linha[2]
            pessoa.secao = int(linha[3])
            eleitores.append(pessoa)
            linha = ler.readline().split("|")
        return eleitores


def escrever_voto(votos, id):
    with open("Votos_" + str(id) + ".md", "w") as escrever:
        escrever.write("|Numero Seçao|Codigo Candidato|Numero Eleitor|\r\n")
        escrever.write("|-|-|-|\r\n")
        for voto in votos:
            escrever.write(
                "|"
                + str(voto.secao)
                + "|"
                + str(voto.codigo_candidadto)
                + "|"
                + str(voto.numero_eleitor)
                + "|\r\n"
            )


def ler_voto(id):
    votos = []
    with open("Votos_" + str(id) + ".md", "r") as ler:
        # Skiping Headers
        ler.readline()
        ler.readline()
        # Reading line 1
        linha = ler.readline().split("|")
        # Loop de leitura
        while len(linha) > 1:
            cedula = voto()
            cedula.secao = int(linha[1])
            cedula.codigo_candidadto = int(linha[2])
            cedula.numero_eleitor = int(linha[3])
            votos.append(cedula)
            linha = ler.readline().split("|")

    return votos


def cadastro_eleitor(qtd):
    eleitores = []
    for i in range(0, qtd):
        pessoa = eleitor()
        while True:
            try:
                pessoa.numero_eleitor = int(input("Insira o numero do eleitor: "))
                if pessoa.numero_eleitor > 0:
                    if not any(
                        eleitor.numero_eleitor == pessoa.numero_eleitor
                        for eleitor in eleitores
                    ):
                        break
                    else:
                        print("Eleitor já cadastrado!")
                else:
                    print("Numero do Eleitor igual ou menor que 0!")
            except Exception:
                print("Valor Inválido!")
        while True:
            try:
                pessoa.nome_eleitor = input("Insira o nome do eleitor: ")
                break
            except Exception:
                print("Valor Inválido!")
        while True:
            try:
                pessoa.secao = int(input("Insira o numero da secao: "))
                break
            except Exception:
                print("Valor Inválido!")

        eleitores.append(pessoa)
    eleitores.sort(key=lambda x: x.numero_eleitor)
    escrever_eleitor(eleitores)


def valida_eleitor(eleitores, num):
    for i in range(0, len(eleitores)):
        if eleitores[i].numero_eleitor == num:
            return i
    print("Eleitor não cadastrado!")
    return -1


def valida_secao(id, pessoa):
    match id:
        case 1:
            validas = [1, 3, 4]
        case 2:
            validas = [5, 9, 10]
        case _:
            print("Seção" + id + "não Cadastrada.")
            return False
    for i in range(0, len(validas)):
        if validas[i] == pessoa.secao:
            return True

    print("Eleitor não pertence a esta seção.")
    return False


def cadastro_voto(id, qtd):
    votos = []
    eleitores = ler_eleitor()
    for i in range(0, qtd):
        cedula = voto()

        while True:  # Loop input numero_eleitor
            try:  # Erro de inserir não int
                pessoa = int(input("Insira o numero do eleitor: "))
            except Exception:
                print("Valor Inválido!")
                pessoa = -1
            # Validação do numero_eleitor
            pessoa = valida_eleitor(ler_eleitor(), pessoa)
            print(str(eleitores[pessoa].secao) + "|")
            if pessoa >= 0 and valida_secao(id, eleitores[pessoa]):
                break

        # insere as inormações do eleitor na cedula
        cedula.secao = eleitores[pessoa].secao
        cedula.numero_eleitor = eleitores[pessoa].numero_eleitor
        # input do codigo_candidadto
        while True:
            try:
                pessoa = int(input("Insira o Codigo do Candidato: "))
            except Exception:
                pessoa = -1
            match pessoa:
                case 1 | 2 | 3 | 4:
                    print("Voto Cadastrado.")
                    break
                case _:
                    print("Candidato Inválido!")

        cedula.codigo_candidadto = pessoa
        votos.append(cedula)
    escrever_voto(votos, id)


def agrupa_apuracao():
    votos_1 = ler_voto(1)
    votos_2 = ler_voto(2)

    votos_1.sort(key=lambda x: x.codigo_candidadto)
    votos_2.sort(key=lambda x: x.codigo_candidadto)

    votos = []
    # votos = votos_1 + votos_2 # True python solution
    # votos.sort(key=lambda x: x.codigo_candidadto)
    # Requested In-line
    i = j = k = 0
    while i < len(votos_1) and j < len(votos_2):
        if votos_1[i].codigo_candidadto < votos_2[j].codigo_candidadto:
            votos.append(votos_1[i])
            i += 1
        else:
            votos.append(votos_2[j])
            j += 1
        k += 1
    while i < len(votos_1):
        votos.append(votos_1[i])
        i += 1
        k += 1
    while j < len(votos_2):
        votos.append(votos_2[j])
        j += 1
        k += 1
    escrever_voto(votos, 10)


def cadastra_votacao(qtd):
    while True:
        print("""
|------------------------|
|    Cadastro Votação    |
|------------------------|
| 1 - Cadastro Votação 1 |
| 2 - Cadastro votação 2 |
| 9 - FIM                |
|------------------------|
""")
        # Input error
        try:
            opc = int(input("Opção: "))
        except Exception:
            opc = -1

        # menu Options
        match opc:
            case 1:
                cadastro_voto(1, qtd)
            case 2:
                cadastro_voto(2, qtd)
            case 9:
                clear_terminal()
                break
            case _:
                clear_terminal()
                print("Opção Inválida!")


def contagem():
    votos = ler_voto(10)
    contados = [0, 0, 0, 0]
    for i in range(0, len(votos)):
        contados[votos[i].codigo_candidadto - 1] += 1
    return contados


def print_eleitores():
    eleitores = ler_eleitor()
    text = "|numero_eleitor|secao|nome_eleitor"
    for i in range(0, len(eleitores)):
        text += (
            "\r\n|"
            + str(eleitores[i].numero_eleitor)
            + "|"
            + str(eleitores[i].secao)
            + "|"
            + eleitores[i].nome_eleitor
        )
    print(text)


def print_apuracao():
    votos = ler_voto(10)
    text = "|codigo_candidadto|numero_eleitor|secao"
    for i in range(0, len(votos)):
        text += (
            "\r\n"
            + str(votos[i].codigo_candidadto)
            + "|"
            + str(votos[i].numero_eleitor)
            + "|"
            + str(votos[i].secao)
        )
    print(text)


def estatistica():
    while True:
        clear_terminal()
        print("""
|-----------------------------------|
|          MENU ESTATÍSTICA         |
|-----------------------------------|
| 1 - Vencedor                      |
| 2 - Segundo colocado              |
| 3 - Quantidade de votos em branco |
| 4 - Quantidade de votos nulos     |
| 5 - Mostra eleitores              |
| 6 - Mostra apuração               |
| 9 - FIM                           |
|-----------------------------------|
""")
        colocacao = contagem()
        # Input error
        try:
            opc = int(input("Opção: "))
        except Exception:
            opc = -1

        # menu Options
        match opc:
            case 1:
                if colocacao[0] > colocacao[1]:
                    print("O Vencedor foi Jose.")
                else:
                    print("O Vencedor foi Maria.")
                input("Return (Enter)")
            case 2:
                if colocacao[0] < colocacao[1]:
                    print("O Segundo colocado foi Jose.")
                else:
                    print("O Segundo colocado foi Maria.")
                input("Return (Enter)")
            case 3:
                print("Foram", colocacao[2], "votos em branco.")
                input("Return (Enter)")
            case 4:
                print("Foram", colocacao[3], "votos nulos.")
                input("Return (Enter)")
            case 5:
                print_eleitores()
                input("Return (Enter)")
            case 6:
                print_apuracao()
                input("Return (Enter)")
            case 9:
                clear_terminal()
                break
            case _:
                clear_terminal()
                print("Opção Inválida!")


qtd = 10  # Numero de Eleitores deve ser par
while True:
    print("""
|----------------------------|
|       MENU PRINCIPAL       |
|----------------------------|
| 1 - Cadastra Eleitor       |
| 2 - Cadastra Votação 1 e 2 |
| 3 - Agrupa  Apuração       |
| 4 - Menu  Estatística      |
| 9 - FIM                    |
|----------------------------|
""")
    # Input error
    try:
        opc = int(input("Opção: "))
    except Exception:
        opc = -1

    # menu Options
    match opc:
        case 1:
            cadastro_eleitor(qtd)
        case 2:
            cadastra_votacao(qtd - int(qtd / 2))
        case 3:
            agrupa_apuracao()
        case 4:
            estatistica()
        case 9:
            clear_terminal()
            break
        case _:
            clear_terminal()
            print("Opção Inválida!")
