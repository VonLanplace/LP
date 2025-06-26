import random


class Estatistica:
    def __init__(self) -> None:
        self.set(0, "", 0)
        pass

    def set(self, codigo_cidade, nome_cidade, qtd_acidentes) -> None:
        self.codigo_cidade = codigo_cidade
        self.nome_cidade = nome_cidade
        self.qtd_acidentes = qtd_acidentes
        pass


def print_estatitica(estatisticas):
    print(
        "Codigo ",
        estatisticas.codigo_cidade,
        "|Nome ",
        estatisticas.nome_cidade,
        "|Qtd ",
        estatisticas.qtd_acidentes,
    )


def new_stats():
    stats = Estatistica()
    stats.codigo_cidade = int(input("Insira o Codigo da Cidade:"))
    stats.nome_cidade = input("Insira o Nome da Cidade:")
    stats.qtd_acidentes = int(input("Insira o Qtd de Acidentes:"))
    return stats


def new_stats_random():
    stat = Estatistica()
    stat.codigo_cidade = random.randint(1, 100)
    stat.nome_cidade = ("Cidade_", random.randint(1, 100))
    stat.qtd_acidentes = random.randint(1, 1000)
    return stat


def FCADRASTRAESTATISTICA():
    estatisticas = []
    for i in range(0, 10, 1):
        try:
            estatisticas.append(new_stats())
            print_estatitica(estatisticas[i])
        except Exception as e:
            print(f"An unexpected error occurred: {e}")
            break
    return estatisticas


def PQTDACIDENTES(estatisticas):
    estatisticas.sort(key=lambda x: x.codigo_cidade)
    print("Qtd entre 500 e 100:")
    for i in range(0, len(estatisticas), 1):
        if 500 > estatisticas[i].qtd_acidentes > 100:
            print_estatitica(estatisticas[i])


def PMAIORMENOR(estatisticas):
    estatisticas.sort(key=lambda x: x.qtd_acidentes)
    print("Menor:")
    print_estatitica(estatisticas[0])
    print("Maior:")
    print_estatitica(estatisticas[len(estatisticas) - 1])


def PACIMA(estatisticas):
    estatisticas.sort(key=lambda x: x.codigo_cidade)
    media = 0
    for i in range(0, len(estatisticas), 1):
        media += estatisticas[i].qtd_acidentes
    media /= len(estatisticas)
    print("Media:")
    print(int(media))
    print("Acima da Média:")
    for i in range(0, len(estatisticas), 1):
        if estatisticas[i].qtd_acidentes > media:
            print_estatitica(estatisticas[i])


estatisticas = []
opc = 0
while True:
    try:
        opc = int(
            input("""
|-----------------------------------------------|
|  menu estatística                             |
|-----------------------------------------------|
| Estatísticas de acidentes em 2020             |
| 1 - cadastro estatística                      |
| 2 - consulta por quantidade de acidentes      |
| 3 - consulta por estatísticas de acidentes    |
| 4 - acidentes acima da média das 10 cidades   |
| 9 - finaliza                                  |
|-----------------------------------------------|
Opção: """)
        )
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        opc = -1
    finally:
        match opc:
            case 1:
                estatisticas = FCADRASTRAESTATISTICA()
            case 2:
                PQTDACIDENTES(estatisticas)
            case 3:
                PMAIORMENOR(estatisticas)
            case 4:
                PACIMA(estatisticas)
            case 9:
                break
            case _:
                print("Opção Inválida!")
