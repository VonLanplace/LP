use std::io;

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    clear_console();
    let mut stats = vec![];
    loop {
        println!(
            "
| --------------------------------------------- |
| MENU ESTATÍSTICA                              |
| --------------------------------------------- |
| Estatísticas de acidentes em 2020             |
| 1 - Cadastro Estatística                      |
| 2 - Consulta por quantidade de acidentes      |
| 3 - Consulta por estatísticas de acidentes    |
| 4 - Acidentes acima da média das 10 cidades   |
| 9 - Finaliza                                  |
| --------------------------------------------- |
"
        );
        let opc = input_u32("Opção:");
        clear_console();
        match opc {
            1 => {
                println!("Cadastro de Estatísticas:");
                stats = f_cadastra_estatistica();
            }
            2 => {
                println!("Por Quantidade de Acidentes:");
                p_qtd_acidentes(&stats);
            }
            3 => {
                println!("Por Maior e Menor:");
                stats[0].print();
                stats[stats.len() - 1].print();
            }
            4 => {
                println!("Por Acima da média:");
                p_acima(&stats);
            }
            9 => break,
            _ => println!("Opção Inválida!"),
        };
    }
}

fn f_cadastra_estatistica() -> Vec<Estatistica> {
    let mut stats = vec![];
    for _i in 0..10 {
        let stat = Estatistica::from_values(
            input_u32("Insira o codigo da cidade: ") as i32,
            input_string("Insira o nome da cidade: "),
            input_u32("Insira a quantidade de acidentes da cidade: ") as i32,
        );
        stats.push(stat);
    }
    stats.sort_by_key(|x| x.qtd_acidentes);
}

fn p_qtd_acidentes(estatisticas: &Vec<Estatistica>) {
    for stat in estatisticas {
        if 100 < stat.qtd_acidentes && stat.qtd_acidentes < 500 {
            stat.print();
        }
    }
}

fn p_acima(estatistica: &Vec<Estatistica>) {
    let mut media = 0;
    for stat in estatistica {
        media += stat.qtd_acidentes;
    }
    media /= estatistica.len() as i32;
    println!("A média é {}.", media);
    for stat in estatistica {
        if stat.qtd_acidentes > media {
            stat.print();
        }
    }
}

struct Estatistica {
    codigo_cidade: i32,
    nome_cidade: String,
    qtd_acidentes: i32,
}

impl Estatistica {
    fn print(&self) {
        println!(
            "[{:4.}|{}|{:4.}]",
            self.codigo_cidade, self.nome_cidade, self.qtd_acidentes,
        );
    }

    fn from_values(codigo_cidade: i32, nome_cidade: String, qtd_acidentes: i32) -> Self {
        Estatistica {
            codigo_cidade,
            nome_cidade,
            qtd_acidentes,
        }
    }
}

impl Default for Estatistica {
    fn default() -> Self {
        Estatistica {
            codigo_cidade: 0,
            nome_cidade: String::new(),
            qtd_acidentes: 0,
        }
    }
}

fn input_string(texto: &str) -> String {
    let string: String;
    println!("{}", texto);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error read_line");
    string = input.trim().to_string();
    return string;
}

fn input_u32(texto: &str) -> u32 {
    let int: u32;
    loop {
        println!("{}", texto);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error read_line");
        int = match input.trim().parse() {
            Ok(n) => n,

            Err(_) => {
                println!("Valor inválido!");
                continue;
            }
        };
        return int;
    }
}
