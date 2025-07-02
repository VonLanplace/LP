use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
};

fn main() {
    loop {
        println!(
            "
| ----------------------------- |
| MENU PRINCIPAL                |
| ----------------------------- |
| 1 - Cadastra Contas Correntes |
| 2 - Cadastra movimento        |
| 3 - Gera Contas Atualizadas   |
| 4 - Consulta cadastros        |
| 9 - FIM                       |
| ----------------------------- |"
        );
        let opc = input_i32("opção:");
        clear_console();
        match opc {
            1 => cadastra_contas(),
            2 => cadastra_movimento(),
            3 => gera_atualizada(),
            4 => consulta(),
            9 => break,
            _ => println!("Opção inválida!"),
        };
    }
}
///Cadastra uma conta corrente
fn cadastra_contas() {
    println!("Cadastra Contas:");
    let mut contas = vec![];
    for _i in 0..5 {
        let mut conta = Conta::default();
        // Valida cod conta
        loop {
            let aux = input_i32_p("Insira o número da Conta: ");
            if valida_cadastro_cod(&contas, aux) {
                conta.cod_conta = aux;
                break;
            } else {
                println!("Conta já cadastrada!");
            }
        }
        conta.nome_clientes = input_string("Insira o nome do cliete: ");
        conta.saldo_conta = input_f64_p("Insira o saldo da conta: ");
        conta.limite_conta = input_f64_p("Insira o limite da conta: ");
        loop {
            let aux = input_i32_p("insira o tipo de conta: ");
            if 0 < aux && aux < 5 {
                conta.tipo_conta = aux;
                break;
            } else {
                println!("Tipo inválido!");
            }
        }
        contas.push(conta);
    }
    contas.sort_by_key(|z| z.cod_conta);
    write_contas(contas).expect("Erro de escrita cadastra_contas!!");
}
fn valida_cadastro_cod(contas: &Vec<Conta>, cod_contas: i32) -> bool {
    for conta in contas {
        if conta.cod_conta == cod_contas {
            return true;
        }
    }
    return false;
}

/// Cadastra Movimentos
fn cadastra_movimento() {
    println!("Cadastra Movimentos:");
    let mut movimentos = vec![];
    for _i in 0..10 {
        let mut movimento = Movimento::default();
        movimento.cod_conta = input_i32_p("Insira o codigo da conta: ");
        movimento.valor_movimento = input_f64_p("Insira o valor do movimento: ");
        loop {
            let aux = input_i32("Insira o tipo do movimento: ");
            if 0 < aux && aux < 3 {
                movimento.tipo_movimento = aux;
                break;
            } else {
                println!("Tipo de movimento inválido!!");
            }
        }
        loop {
            let aux = input_i32_p("Insira o status do movimento: ");
            if 0 < aux && aux < 3 {
                movimento.status = aux;
                break;
            } else {
                println!("Status de movimento Inválido!!");
            }
        }
        movimentos.push(movimento);
    }
    movimentos.sort_by_key(|z| z.cod_conta);
    write_movimento(movimentos).expect("Erro write_movimento");
}

/// Gera Atualizadas
fn gera_atualizada() {
    println!("Gerar Atualizadas");
    let movimentos = read_movimento().expect("Erro de Leitura movimento!");
    let mut atualizadas = read_contas();

    let mut i = 0;
    let mut j = 0;
    while i < movimentos.len() && j < atualizadas.len() {
        if movimentos[i].cod_conta == atualizadas[j].cod_conta {
            if movimentos[i].status != 1 {
                match movimentos[i].tipo_movimento {
                    1 => {
                        atualizadas[j] =
                            criterio_limite(atualizadas[j].clone(), movimentos[i].valor_movimento);
                    }
                    2 => {
                        if atualizadas[j].saldo_conta > movimentos[i].valor_movimento {
                            atualizadas[j].saldo_conta -= movimentos[i].valor_movimento;
                        } else {
                            atualizadas[j] = criterio_limite(
                                atualizadas[j].clone(),
                                movimentos[i].valor_movimento,
                            );
                        }
                    }
                    _ => println!("Tipo de movimento não cadastrado!"),
                };
            }
            i += 1;
        } else if movimentos[i].cod_conta < atualizadas[j].cod_conta {
            i += 1;
        } else {
            j += 1;
        }
    }
    write_atualizadas(atualizadas).expect("Erro write_atualizadas");
}
fn criterio_limite(mut contas: Conta, mut valor: f64) -> Conta {
    match contas.tipo_conta {
        1 => contas.saldo_conta -= valor,
        2 => contas.limite_conta -= valor,
        3 => {
            if valor <= contas.limite_conta {
                contas.limite_conta -= valor;
            } else {
                valor -= contas.limite_conta;
                contas.limite_conta = 0.0;
                contas.saldo_conta -= valor;
            }
        }
        4 => {
            if valor <= contas.limite_conta {
                contas.limite_conta -= valor;
            } else {
                valor -= contas.limite_conta;
                contas.limite_conta = 0.0;
                contas.saldo_conta -= valor;
            }
        }
        _ => println!("Tipo inválido"),
    }
    contas
}
///
///Consulta os arquivos
fn consulta() {
    loop {
        println!(
            "
| -------------------- |
| Consulta Cadastros   |
| -------------------- |
| 1 - Contas Correntes |
| 2 - Movimento        |
| 3 - Atualizado       |
| 9 - FIM              |
| -------------------- |"
        );
        let opc = input_i32("opção:");
        clear_console();
        match opc {
            1 => consulta_contas(),
            2 => consulta_movimento(),
            3 => consulta_atualizadas(),
            9 => break,
            _ => println!("Opção inválida!"),
        };
    }
}
fn consulta_atualizadas() {
    let contas = read_atualizadas();
    println!("Consulta das Atualizadas:");
    for con in contas {
        con.print();
    }
}
fn consulta_contas() {
    let contas = read_contas();
    println!("Consulta dos Contas:");
    for con in contas {
        con.print();
    }
}
fn consulta_movimento() {
    let movimentos = read_movimento().expect("Erro de leitura de Movimento!!");
    println!("Consulta dos Movimentos:");
    for muv in movimentos {
        muv.print();
    }
}
//struct
//

///Conta
#[derive(Debug, Clone)]
struct Conta {
    cod_conta: i32,
    nome_clientes: String,
    saldo_conta: f64,
    limite_conta: f64,
    tipo_conta: i32,
}
impl Default for Conta {
    fn default() -> Self {
        Conta {
            cod_conta: 0,
            nome_clientes: "".to_string(),
            saldo_conta: 0.0,
            limite_conta: 0.0,
            tipo_conta: 0,
        }
    }
}

impl Conta {
    fn print(&self) {
        println!(
            "[{},\"{}\",{:.2},{:.2},{}]",
            self.cod_conta,
            self.nome_clientes,
            self.saldo_conta,
            self.limite_conta,
            self.tipo_conta
        );
    }
    /*
    fn from_value(
        cod_conta: i32,
        nome_clientes: String,
        saldo_conta: f64,
        limite_conta: f64,
        tipo_conta: i32,
    ) -> Self {
        Conta {
            cod_conta,
            nome_clientes,
            saldo_conta,
            limite_conta,
            tipo_conta,
        }
    }
    */
}
fn write_atualizadas(contas: Vec<Conta>) -> std::io::Result<()> {
    write_contas_correntes(contas, "Atualizadas.md")
}
fn write_contas(contas: Vec<Conta>) -> std::io::Result<()> {
    write_contas_correntes(contas, "Contas.md")
}
fn write_contas_correntes(contas: Vec<Conta>, file: &str) -> std::io::Result<()> {
    let mut escrever = BufWriter::new(File::create(file)?);
    escrever.write_all(b"|cod_conta|nome_clientes|saldo_conta|limite_conta|tipo_conta|")?;
    escrever.write_all(b"\n|-|-|-|-|-|")?;
    for conta in contas {
        write!(
            escrever,
            "\n|{}|{}|{}|{}|{}|",
            conta.cod_conta,
            conta.nome_clientes,
            conta.saldo_conta,
            conta.limite_conta,
            conta.tipo_conta
        )?;
    }
    Ok(())
}
fn read_atualizadas() -> Vec<Conta> {
    read_contas_correntes("Atualizadas.md")
}
fn read_contas() -> Vec<Conta> {
    read_contas_correntes("Contas.md")
}
fn read_contas_correntes(file_name: &str) -> Vec<Conta> {
    let msg = "Erro de Parse Read Contas";
    let tabela = ler_tabela(file_name);
    let mut contas = vec![];
    for linha in tabela {
        let mut conta = Conta::default();
        conta.cod_conta = linha[0].parse().expect(msg);
        conta.nome_clientes = linha[1].to_string();
        conta.saldo_conta = linha[2].parse().expect(msg);
        conta.limite_conta = linha[3].parse().expect(msg);
        conta.tipo_conta = linha[4].parse().expect(msg);
        contas.push(conta);
    }
    contas
}
///Movimento
#[derive(Debug, Clone)]
struct Movimento {
    cod_conta: i32,
    valor_movimento: f64,
    tipo_movimento: i32,
    status: i32,
}
impl Default for Movimento {
    fn default() -> Self {
        Movimento {
            cod_conta: 0,
            valor_movimento: 0.0,
            tipo_movimento: 0,
            status: 0,
        }
    }
}
impl Movimento {
    fn print(&self) {
        println!(
            "[{},{:.2},{},{}]",
            self.cod_conta, self.valor_movimento, self.tipo_movimento, self.status
        );
    }
    /*
    fn from_value(cod_conta: i32, valor_movimento: f64, tipo_movimento: i32, status: i32) -> Self {
        Movimento {
            cod_conta,
            valor_movimento,
            tipo_movimento,
            status,
        }
    }
    */
}
fn write_movimento(movimentos: Vec<Movimento>) -> std::io::Result<()> {
    let mut escrever = BufWriter::new(File::create("Movimentos.md")?);
    escrever.write_all(b"|cod_conta|valor_movimento|tipo_movimento|status|")?;
    escrever.write_all(b"\n|-|-|-|-|-|")?;
    for movimento in movimentos {
        write!(
            escrever,
            "\n|{}|{}|{}|{}|",
            movimento.cod_conta,
            movimento.valor_movimento,
            movimento.tipo_movimento,
            movimento.status
        )?;
    }
    Ok(())
}
fn read_movimento() -> io::Result<Vec<Movimento>> {
    let msg = "Erro de Parse Read Movimentos";
    let tabela = ler_tabela("Movimentos.md");
    let mut movimentos = vec![];
    for linha in tabela {
        let mut movimento = Movimento::default();
        movimento.cod_conta = linha[0].parse().expect(msg);
        movimento.valor_movimento = linha[1].parse().expect(msg);
        movimento.tipo_movimento = linha[2].parse().expect(msg);
        movimento.status = linha[3].parse().expect(msg);
        movimentos.push(movimento);
    }
    Ok(movimentos)
}
//Utils
//
///Input for a integer = i32
fn input_i32(text: &str) -> i32 {
    let integer;
    loop {
        println!("{}", text);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input i32");
        integer = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Valor inválida!");
                println!("Valor Não numérico!");
                continue;
            }
        };
        break;
    }
    integer
}
///Input for a integer = i32 positivo
fn input_i32_p(text: &str) -> i32 {
    let integer;
    loop {
        println!("{}", text);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input i32");
        integer = match input.trim().parse() {
            Ok(n) => {
                if n >= 0 {
                    println!("Valor inválido!");
                    println!("Valor Abaixo de 0!");
                    continue;
                } else {
                    n
                }
            }
            Err(_) => {
                println!("Valor inválida!");
                println!("Valor Não numérico!");
                continue;
            }
        };
        break;
    }
    integer
}
///Input for a float = f64 positivo
fn input_f64_p(text: &str) -> f64 {
    let float;
    loop {
        println!("{}", text);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input i32");
        float = match input.trim().parse() {
            Ok(n) => {
                if n >= 0.0 {
                    println!("Valor inválido!");
                    println!("Valor Abaixo de 0!");
                    continue;
                } else {
                    n
                }
            }
            Err(_) => {
                println!("Valor inválida!");
                println!("Valor Não numérico!");
                continue;
            }
        };
        break;
    }
    float
}
///Inpnut for a String
fn input_string(text: &str) -> String {
    let string;
    println!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input String");
    string = input.trim().to_string();
    string
}
/// Reads the contents of a File into a String
fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect(&("Erro de Leitura - ".to_owned() + file_name))
}
/// lê um arquivo .md contendo uma tabela em um vetor de strings retirando o cabeçalho incluindo a
/// segunda linha de traços da tabela
fn ler_tabela(file_name: &str) -> Vec<Vec<String>> {
    let monolito = read_file(file_name);
    let mut tabela = vec![];
    {
        for linha in monolito.trim().split("\n") {
            let mut casas = vec![];
            for casa in linha.trim().split("|") {
                casas.push(casa.to_owned());
            }
            casas.pop();
            casas.remove(0);
            tabela.push(casas);
        }
        tabela.remove(0);
        tabela.remove(0);
    }
    tabela
}
///Clear terminal
fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
