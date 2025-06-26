import random


class Votacao:
    # |NumeroSeção|NumeroCandidato|
    def __init__(self) -> None:
        self.set(0, 0)
        pass

    def set(self, numero_secao, numero_candidato) -> None:
        self.numero_secao = numero_secao
        self.numero_candidato = numero_candidato
        pass


def print_votacao(voto):
    print(
        "|Numero Seção: ",
        voto.numero_secao,
        "|Numero Candidato: ",
        voto.numero_candidato,
        "|",
    )


def FCADRASTRAVOTAÇÃO():
    votos = []
    for i in range(0, 200):
        voto = Votacao()
        voto.numero_secao = random.randint(0, 10)
        voto.numero_candidato = random.randint(0, 300)
        votos.append(voto)
    return votos


def FGRAVAVOTAÇÃO(votos):
    with open("Votação2021.txt", "w") as escrever:
        for i in range(0, len(votos)):
            escrever.write(
                "|"
                + str(votos[i].numero_secao)
                + "|"
                + str(votos[i].numero_candidato)
                + "|\r\n"
            )


def ler_votacao():
    votos = []
    with open("Votação2021.txt", "r") as ler:
        while True:
            line = ler.readline().split("|")
            if len(line) > 1:
                voto = Votacao()
                voto.numero_secao = int(line[1])
                voto.numero_candidato = int(line[2])
                votos.append(voto)
            else:
                break
    return votos


def indicador_eleitores_secao():
    votos = ler_votacao()
    soma = 0
    j = 0
    for i in range(0, len(votos)):
        if j != votos[i].numero_secao:
            print("Seção", str(j), ":", soma, "eleitores!")
            soma = 0
            j += 1
        soma += 1


def indicadores_votos_candidatos():
    votos = ler_votacao()
    votos.sort(key=lambda x: x.numero_candidato)

    candidatos = []
    qtd_votos = []
    j = -1
    k = -1
    for i in range(0, len(votos)):
        if j != votos[i].numero_candidato:
            j = votos[i].numero_candidato
            k += 1
            candidatos.append(votos[i].numero_candidato)
            qtd_votos.append(0)
        qtd_votos[k] += 1

    print("Votos por Candidatos: ")
    for i in range(0, len(qtd_votos)):
        print("Candidato", candidatos[i], "numero de Votos", qtd_votos[i])


def indicadores_top():
    votos = ler_votacao()
    votos.sort(key=lambda x: x.numero_candidato)

    candidatos = []
    qtd_votos = []
    j = -1
    k = -1
    for i in range(0, len(votos)):
        if j != votos[i].numero_candidato:
            j = votos[i].numero_candidato
            k += 1
            candidatos.append(votos[i].numero_candidato)
            qtd_votos.append(0)
        qtd_votos[k] += 1

    votos = []
    for i in range(0, len(candidatos)):
        voto = Votacao()
        voto.numero_secao = qtd_votos[i]
        voto.numero_candidato = candidatos[i]
        votos.append(voto)

    votos.sort(key=lambda x: x.numero_secao, reverse=True)

    print("Classificação: ")
    for i in range(0, 10):
        print(
            str(i + 1),
            "° Candidato N°",
            votos[i].numero_candidato,
            "Numero de Votos",
            votos[i].numero_secao,
        )


def indicador_maior_menor_eleitores():
    votos = ler_votacao()
    somas = []
    j = -1
    for i in range(0, len(votos)):
        if j < votos[i].numero_secao:
            j = j + 1
            somas.append(0)
        somas[j] = somas[j] + 1

    max = 0
    min = 0
    for i in range(0, len(somas)):
        if i == 0:
            min = i
            max = i

        if somas[i] > somas[max]:
            max = i
        elif somas[i] < somas[min]:
            min = i

    print("Maior: ")
    print(" Seção", max, "com", somas[max], "eleitores.")
    print("Menor: ")
    print(" Seção", min, "com", somas[min], "eleitores.")


def Indicadores():
    while True:
        print("""
|----------------------------------------------------|
|               Mostrar Indicadores                  |
|----------------------------------------------------|
| Estatísticas de Votação em 2021                    |
| 1 – Quantidade Eleitores por Seção                 |
| 2 – Seção com Maior e Menor número de Eleitores    |
| 3 – Quantidade de votos por candidato              |
| 4 – 10 primeiros colocadas (nro cand. e qtd votos) |
| 9 – Finaliza consulta                              |
|----------------------------------------------------|
""")
        try:
            opc = int(input("Opção: "))
        except Exception as e:
            print(f"An unexpected error occurred: {e}")
            opc = -1
        match opc:
            case 1:
                indicador_eleitores_secao()
            case 2:
                indicador_maior_menor_eleitores()
            case 3:
                indicadores_votos_candidatos()
            case 4:
                indicadores_top()
            case 9:
                break
            case _:
                print("Opção Inválida!")


votos = []
while True:
    print("""
|-----------------------------------|
|       SISTEMA DE VOTAÇÃO          |
|-----------------------------------|
| 1 – Carregar Seção/Número Eleitor |
| 2 – Classificar por Seção         |
| 3 – Gravar Registros              |
| 4 – Mostrar Indicadores           |
| 9 – Finalizar                     |
|-----------------------------------|""")
    try:
        opc = int(input("Opção: "))
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        opc = -1
    match opc:
        case 1:
            votos = FCADRASTRAVOTAÇÃO()
        case 2:
            # FCLASSIFICASEÇÃO(votos)
            votos.sort(key=lambda x: x.numero_secao)
        case 3:
            FGRAVAVOTAÇÃO(votos)
        case 4:
            Indicadores()
        case 9:
            break
        case _:
            print("Opção Inválida!")
