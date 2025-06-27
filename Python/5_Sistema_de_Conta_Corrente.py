import os


def clear_terminal():
    if os.name == "nt":
        os.system("cls")
    else:
        os.system("clear")


class Conta:
    # | codContaC | nomeClientesC | saldoContaC | LimitecontaC |tipoContaC|
    # | codContaA | nomeClientesA | saldoContaA | LimitecontaA |tipoContaA|
    def __init__(self) -> None:
        self.new(0, "", 0.0, 0.0, 0)
        pass

    def new(
        self, codigo_conta, nome_cliente, saldo_conta, limite_conta, tipo_conta
    ) -> None:
        self.codigo_conta = codigo_conta
        self.nome_cliente = nome_cliente
        self.saldo_conta = saldo_conta
        self.limite_conta = limite_conta
        self.tipo_conta = tipo_conta
        pass


def escrever_contas(contas):
    contas.sort(key=lambda x: x.codigo_conta)
    with open("Contas.md", "w") as escrever:
        escrever.write("|Cod_Conta|Nome_Clientes|Saldo_Conta|Limite_Conta|Tipo_Conta|")
        escrever.write("\r\n|-|-|-|-|-|")
        for i in range(0, len(contas)):
            escrever.write(
                "\r\n|"
                + str(contas[i].codigo_conta)
                + "|"
                + str(contas[i].nome_cliente)
                + "|"
                + str(contas[i].saldo_conta)
                + "|"
                + str(contas[i].limite_conta)
                + "|"
                + str(contas[i].tipo_conta)
                + "|"
            )


def ler_contas():
    contas = []
    with open("Contas.md", "r") as ler:
        ler.readline()
        ler.readline()
        linha = ler.readline().split("|")
        while len(linha) > 1:
            conta = Conta()
            conta.codigo_conta = int(linha[1])
            conta.nome_cliente = linha[2]
            conta.saldo_conta = float(linha[3])
            conta.limite_conta = float(linha[4])
            conta.tipo_conta = int(linha[5])
            contas.append(conta)
            linha = ler.readline().split("|")
    return contas


def escrever_atualizadas(contas):
    contas.sort(key=lambda x: x.codigo_conta)
    with open("Atualizadas.md", "w") as escrever:
        escrever.write("|Cod_Conta|Nome_Clientes|Saldo_Conta|Limite_Conta|Tipo_Conta|")
        escrever.write("\r\n|-|-|-|-|-|")
        for i in range(0, len(contas)):
            escrever.write(
                "\r\n|"
                + str(contas[i].codigo_conta)
                + "|"
                + str(contas[i].nome_cliente)
                + "|"
                + str(contas[i].saldo_conta)
                + "|"
                + str(contas[i].limite_conta)
                + "|"
                + str(contas[i].tipo_conta)
                + "|"
            )


def ler_atualizadas():
    contas = []
    with open("Atualizadas.md", "r") as ler:
        ler.readline()
        ler.readline()
        linha = ler.readline().split("|")
        while len(linha) > 1:
            conta = Conta()
            conta.codigo_conta = int(linha[1])
            conta.nome_cliente = linha[2]
            conta.saldo_conta = float(linha[3])
            conta.limite_conta = float(linha[4])
            conta.tipo_conta = int(linha[5])
            contas.append(conta)
            linha = ler.readline().split("|")
    return contas


class Movimento:
    # | codContaM | ValorMovimentoM | TipoMovimentoM |statusM|
    def __init__(self) -> None:
        self.new(0, 0, 0, 0)
        pass

    def new(
        self, codigo_conta, valor_movimento, tipo_movimento, status_movimento
    ) -> None:
        self.codigo_conta = codigo_conta
        self.valor_movimento = valor_movimento
        self.tipo_movimento = tipo_movimento
        self.status_movimento = status_movimento
        pass


def escrever_movimentos(movimentos):
    movimentos.sort(key=lambda x: x.codigo_conta)
    with open("Movimento.md", "w") as escrever:
        escrever.write("|Cod_Conta|Valor_Movimento|Tipo_Movimento|Status_Movimento|")
        escrever.write("\r\n|-|-|-|-|-|")
        for i in range(0, len(movimentos)):
            escrever.write(
                "\r\n|"
                + str(movimentos[i].codigo_conta)
                + "|"
                + str(movimentos[i].valor_movimento)
                + "|"
                + str(movimentos[i].tipo_movimento)
                + "|"
                + str(movimentos[i].status_movimento)
                + "|"
            )


def ler_movimentos():
    movimentos = []
    with open("Movimento.md", "r") as ler:
        ler.readline()
        ler.readline()
        linha = ler.readline().split("|")
        while len(linha) > 1:
            movimento = Movimento()
            movimento.codigo_conta = int(linha[1])
            movimento.valor_movimento = float(linha[2])
            movimento.tipo_movimento = int(linha[3])
            movimento.status_movimento = int(linha[4])
            movimentos.append(movimento)
            linha = ler.readline().split("|")
    return movimentos


def print_movimentos():
    movimentos = ler_movimentos()
    print("|Cod_Conta|Valor_Movimento|Tipo_Movimento|Status_Movimento|")
    print("|---------|---------------|--------------|----------------|")
    for i in range(0, len(movimentos)):
        print(
            "|",
            movimentos[i].codigo_conta,
            "|",
            movimentos[i].valor_movimento,
            "|",
            movimentos[i].tipo_movimento,
            "|",
            movimentos[i].status_movimento,
            "|",
        )
    input("Return (Enter)")


def print_atualizadas():
    contas = ler_atualizadas()
    print("|Cod_Conta|Nome_Clientes|Saldo_Conta|Limite_Conta|Tipo_Conta|")
    print("|---------|-------------|-----------|------------|----------|")
    for i in range(0, len(contas)):
        print(
            "|",
            contas[i].codigo_conta,
            "|",
            contas[i].nome_cliente,
            "|",
            contas[i].saldo_conta,
            "|",
            contas[i].limite_conta,
            "|",
            contas[i].tipo_conta,
            "|",
        )
    input("Return (Enter)")


def print_contas():
    contas = ler_contas()
    print("|Cod_Conta|Nome_Clientes|Saldo_Conta|Limite_Conta|Tipo_Conta|")
    print("|---------|-------------|-----------|------------|----------|")
    for i in range(0, len(contas)):
        print(
            "|",
            contas[i].codigo_conta,
            "|",
            contas[i].nome_cliente,
            "|",
            contas[i].saldo_conta,
            "|",
            contas[i].limite_conta,
            "|",
            contas[i].tipo_conta,
            "|",
        )
    input("Return (Enter)")


def consultar_cadastros():
    while True:
        print("""
|----------------------|
|  Consulta Cadastros  |
|----------------------|
| 1 - Contas Correntes |
| 2 - Movimento        |
| 3 - Atualizado       |
| 9 - FIM              |
|----------------------|
""")
        try:
            opc = int(input("Opção: "))
        except Exception:
            opc = -1
        # Menu Buttons
        clear_terminal()
        match opc:
            case 1:
                print_contas()
                input("Return (Enter)")
            case 2:
                print_movimentos()
                input("Return (Enter)")
            case 3:
                print_atualizadas()
                input("Return (Enter)")
            case 9:
                break
            case _:
                print("Opção Inválida")


def validar_codigo(aux, contas):
    contas.sort(key=lambda x: x.codigo_conta)
    for i in range(0, len(contas)):
        if aux == contas[i].codigo_conta:
            return True
    return False


def cadastra_contas(qtd):
    contas = []
    for i in range(0, qtd):
        conta = Conta()

        # Codigo input loop
        while True:
            # Test valid input
            try:
                aux = int(input("Insira o código da Conta: "))
            except Exception:
                aux = -1
            #
            if aux > 0 and not validar_codigo(aux, contas):
                conta.codigo_conta = aux
                break
            elif validar_codigo(aux, contas):
                print("Conta já cadastrada!\r\nInsira outro Código!")
            else:
                print("Valor inválido!")

        # Nome input
        conta.nome_cliente = input("Insira o nome do Cliente: ")

        # Saldo input loop
        while True:
            # Test valid input
            try:
                aux = float(input("Insira o saldo da Conta: "))
            except Exception:
                aux = -1.0
            if aux > 0:
                conta.saldo_conta = aux
                break
            else:
                print("Valor inválido!")

        # Tipo input loop
        while True:
            # Test valid input
            try:
                aux = int(input("Insira o Tipo da Conta: "))
            except Exception:
                aux = -1
            if 5 > aux > 0:
                conta.tipo_conta = aux
                break
            else:
                print("Valor inválido!")

        # Limite input loop
        while True:
            # Test valid input
            try:
                aux = float(input("Insira o limite da Conta: "))
            except Exception:
                aux = 0.0
            if aux > 0:
                conta.limite_conta = aux
                break
            else:
                print("Valor inválido!")

        contas.append(conta)
    escrever_contas(contas)


def cadastra_movimento(qtd):
    movimentos = []
    for i in range(0, qtd):
        movimento = Movimento()

        # Codigo input loop
        while True:
            # Test valid input
            try:
                aux = int(input("Insira o código da Conta: "))
            except Exception:
                aux = -1
            #
            if aux > 0 and validar_codigo(aux, ler_contas()):
                movimento.codigo_conta = aux
                break
            elif not validar_codigo(aux, ler_contas()):
                print("Conta não cadastrada!\r\nInsira outro Código!")
            else:
                print("Valor inválido!")

        # Valor input loop
        while True:
            # Test valid input
            try:
                aux = float(input("Insira o valor do Movimento: "))
            except Exception:
                aux = -1.0
            if aux > 0:
                movimento.valor_movimento = aux
                break
            else:
                print("Valor inválido!")

        # Tipo input loop
        while True:
            # Test valid input
            try:
                aux = int(input("Insira o Tipo do Movimento: "))
            except Exception:
                aux = -1
            if 3 > aux > 0:
                movimento.tipo_movimento = aux
                break
            else:
                print("Valor inválido!")

        # Status input loop
        while True:
            # Test valid input
            try:
                aux = int(input("Insira o Status do Movimento: "))
            except Exception:
                aux = -1
            if 3 > aux > 0:
                movimento.status_movimento = aux
                break
            else:
                print("Valor inválido!")

        movimentos.append(movimento)
    escrever_movimentos(movimentos)


def calc_limite(conta, movimento):
    match conta.tipo_conta:
        case 1:
            conta.saldo_conta -= movimento.valor_movimento
        case 2:
            conta.limite_conta -= movimento.valor_movimento
        case 3:
            if conta.limite_conta > movimento.valor_movimento:
                conta.limite_conta -= movimento.valor_movimento
            else:
                aux = conta.limite_conta - movimento.valor_movimento
                conta.saldo_conta += aux
        case 4:
            if conta.limite_conta > movimento.valor_movimento:
                conta.limite_conta -= movimento.valor_movimento
            else:
                aux = conta.limite_conta - movimento.valor_movimento
                conta.saldo_conta += aux
        case _:
            print()
    return conta


def calc_atualizada(conta, movimento):
    if movimento.status_movimento != 2:  # Em Aberto
        if movimento.tipo_movimento == 1:  # Crédito
            conta = calc_limite(conta, movimento)
        else:  # Débito
            if (
                conta.saldo_conta > movimento.valor_movimento
            ):  # Tem valor em conta para pagar
                conta.saldo_conta -= movimento.valor_movimento
            else:  # Não tem valor em conta para pagar
                conta = calc_limite(conta, movimento)
    return conta


def atualizar_contas():
    movimentos = ler_movimentos()
    contas = ler_contas()

    i = j = 0
    while i < len(movimentos) and j < len(contas):
        if movimentos[i].codigo_conta == contas[j].codigo_conta:
            contas[j] = calc_atualizada(contas[j], movimentos[i])
        elif movimentos[i].codigo_conta > contas[j].codigo_conta:
            j += 1
        elif movimentos[i].codigo_conta < contas[j].codigo_conta:
            i += 1
    escrever_atualizadas(contas)


while True:
    print("""
|-------------------------------|
|         MENU PRINCIPAL        |
|-------------------------------|
| 1 - Cadastra Contas Correntes |
| 2 - Cadastra movimento        |
| 3 - Gera Contas Atualizadas   |
| 4 - Consulta cadastros        |
| 9 - FIM                       |
|-------------------------------|
""")
    try:
        opc = int(input("Opção: "))
    except Exception:
        opc = -1
    # Menu Buttons
    clear_terminal()
    match opc:
        case 1:
            cadastra_contas(5)
        case 2:
            cadastra_movimento(10)
        case 3:
            atualizar_contas()
        case 4:
            consultar_cadastros()
        case 9:
            break
        case _:
            print("Opção Inválida")
