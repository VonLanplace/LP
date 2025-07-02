use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    vec,
};

fn main() {
    clear_console();
    let mut qtd = 4;
    if qtd % 2 == 1 {
        qtd += 1;
    }
    loop {
        println!(
            "
| -------------------------- |
| MENU PRINCIPAL             |
| -------------------------- |
| 1 - Cadastra Eleitor       |
| 2 - Cadastra Votação 1 e 2 |
| 3 - Agrupa  Apuração       |
| 4 - Menu  Estatística      |
| 9 - FIM                    |
| -------------------------- |"
        );
        let opc = input_i32("Opção:");
        clear_console();
        match opc {
            1 => cadastra_eleitor(qtd),
            2 => menu_cadastro(qtd / 2),
            3 => agrupa_apuração(),
            4 => menu_estatistica(),
            9 => break,
            _ => println!("Opção inválida"),
        };
    }
}
/// Cadastra os eleitores e salva em um arquivo Eleitores.md
fn cadastra_eleitor(qtd: usize) {
    let mut eleitores: Vec<Eleitor> = vec![];
    for _i in 0..qtd {
        let mut eleitor = Eleitor::default();
        loop {
            let aux = input_i32("Insira o número do eleitor:");
            let mut cadastrado = false;
            for pessoa in &eleitores {
                if pessoa.numero_eleitor == aux {
                    cadastrado = true;
                    break;
                }
            }
            if !cadastrado {
                eleitor.numero_eleitor = aux;
                break;
            } else {
                println!("Eleitor número {} já cadastrado!", aux);
            }
        }
        eleitor.nome_eleitor = input_string("Insira o nome do eleitor:");
        loop {
            let aux = input_i32("Insira o número da seção:");
            if existe_secao(aux) {
                eleitor.secao = aux;
                break;
            } else {
                println!("Seção Inválida!");
            }
        }
        eleitores.push(eleitor);
    }
    eleitores.sort_by_cached_key(|f| f.numero_eleitor);
    write_eleitores(eleitores).expect("Erro de Cadastro - Eleitores");
}
fn existe_secao(secao: i32) -> bool {
    let validas = [1, 3, 4, 5, 9, 10];
    for lugar in validas {
        if lugar == secao {
            return true;
        }
    }
    return false;
}
/// Escreve o arquivo Eleitores.md
fn write_eleitores(eleitores: Vec<Eleitor>) -> std::io::Result<()> {
    let mut escrever = BufWriter::new(File::create("Eleitores.md")?);
    escrever.write_all(b"|numero_eleitor|nome_eleitor|secao|")?;
    escrever.write_all(b"\n|-|-|-|")?;
    for eleitor in eleitores {
        write!(
            escrever,
            "\n|{}|{}|{}|",
            eleitor.numero_eleitor, eleitor.nome_eleitor, eleitor.secao,
        )?;
    }
    Ok(())
}
///Lê o arquivo Eleitores.md
fn read_eleitores() -> Vec<Eleitor> {
    let tabela = ler_tabela("Eleitores.md");
    let mut eleitores = vec![];
    for i in 0..tabela.len() {
        eleitores.push(Eleitor::from_values(
            tabela[i][0].parse().expect(""),
            tabela[i][1].to_string(),
            tabela[i][2].parse().expect(""),
        ));
    }
    eleitores
}
///Agrupa os arquivos de votação em um arquivo de ap
fn agrupa_apuração() {
    let mut apuracao = vec![];
    {
        let mut votacao_1 = ler_votacao("Votacao_1.md");
        let mut votacao_2 = ler_votacao("Votacao_2.md");
        votacao_1.sort_by_key(|z| z.cod_candidato);
        votacao_2.sort_by_key(|z| z.cod_candidato);

        let mut i = 0;
        let mut j = 0;
        while i < votacao_1.len() && j < votacao_2.len() {
            if votacao_1[i].cod_candidato < votacao_2[j].cod_candidato {
                apuracao.push(Voto::from_values(
                    votacao_1[i].secao,
                    votacao_1[i].cod_candidato,
                    votacao_1[i].numero_eleitor,
                ));
                i += 1;
            } else {
                apuracao.push(Voto::from_values(
                    votacao_2[j].secao,
                    votacao_2[j].cod_candidato,
                    votacao_2[j].numero_eleitor,
                ));
                j += 1;
            }
        }
        while i < votacao_1.len() {
            apuracao.push(Voto::from_values(
                votacao_1[i].secao,
                votacao_1[i].cod_candidato,
                votacao_1[i].numero_eleitor,
            ));
            i += 1;
        }
        while j < votacao_2.len() {
            apuracao.push(Voto::from_values(
                votacao_2[j].secao,
                votacao_2[j].cod_candidato,
                votacao_2[j].numero_eleitor,
            ));
            j += 1;
        }
    }
    write_votacao(apuracao, "Apuração.md").expect("write_votacao agrupa_apuração");
}
/// Cadastro de Votação
fn menu_cadastro(qtd: usize) {
    loop {
        println!(
            "
| ---------------------- |
| Cadastro Votação 1,2   |
| ---------------------- |
| 1 - Cadastro Votação 1 |
| 2 - Cadastro votação 2 |
| 9 - FIM                |
| ---------------------- |"
        );
        let opc = input_i32("Opção:");
        clear_console();
        match opc {
            1 => cadastra_votacao(1, qtd),
            2 => cadastra_votacao(2, qtd),
            9 => break,
            _ => println!("Opção inválida"),
        };
    }
}
/// cadastro de votção a partir do numero da votação e Quantidade de eleitores
fn cadastra_votacao(id: i32, qtd: usize) {
    let mut votos = vec![];
    let mut eleitores = read_eleitores();

    for _i in 0..qtd {
        let mut voto = Voto::default();
        //Teste Cadastro Eleitor
        loop {
            let index;
            loop {
                let aux = valida_eleitor(
                    &eleitores,
                    0,
                    (eleitores.len() as i32 - 1) as usize,
                    input_i32("Insira o numero do eleitor: "),
                );
                if aux != -1 {
                    index = aux as usize;
                    break;
                } else {
                    println!("Eleitor não cadastrado!");
                }
            }
            if valida_secao(eleitores[index].secao, id) {
                voto.numero_eleitor = eleitores[index].numero_eleitor;
                voto.secao = eleitores[index].secao;
                eleitores.remove(index);
                break;
            } else {
                println!("Seção inválida!!");
            }
        }

        // Teste codigo candidato
        loop {
            let aux = input_i32("Insira o codigo do candidato: ");
            if 5 > aux && aux > 0 {
                voto.cod_candidato = aux;
                break;
            } else {
                println!("Codigo de candidato inválido!");
            }
        }
        // Aloca cod candidato
        votos.push(voto);
    }
    //Escreve o arquivo
    votos.sort_by_key(|z| z.cod_candidato);
    write_votacao(votos, &("Votacao_".to_owned() + &id.to_string() + ".md"))
        .expect("Erro escrita Votação");
}
fn valida_eleitor(eleitores: &Vec<Eleitor>, min: usize, max: usize, numero_eleitor: i32) -> i32 {
    if max > min {
        let mid = min + max / 2;
        if eleitores[mid].numero_eleitor < numero_eleitor {
            valida_eleitor(eleitores, mid, max, numero_eleitor)
        } else if eleitores[mid].numero_eleitor > numero_eleitor {
            valida_eleitor(eleitores, min, mid, numero_eleitor)
        } else {
            mid as i32
        }
    } else {
        -1
    }
}
fn valida_secao(secao: i32, id: i32) -> bool {
    let validas;
    match id {
        1 => validas = [1, 3, 4],
        2 => validas = [5, 9, 10],
        _ => {
            println!("id da votação não cadastrado!!");
            return false;
        }
    };
    for lugar in validas {
        if lugar == secao {
            return true;
        }
    }
    return false;
}
/// Escreve votação e apuração
fn write_votacao(votos: Vec<Voto>, file_name: &str) -> std::io::Result<()> {
    let mut escrever = BufWriter::new(File::create(file_name)?);
    escrever.write_all(b"|secao|cod_candidato|numero_eleitor|")?;
    escrever.write_all(b"\n|-|-|-|")?;
    for voto in votos {
        write!(
            escrever,
            "\n|{}|{}|{}|",
            voto.secao, voto.cod_candidato, voto.numero_eleitor
        )?;
    }
    Ok(())
}
/// Ler Votação
fn ler_votacao(file_name: &str) -> Vec<Voto> {
    let tabela = ler_tabela(file_name);
    let mut votos = vec![];
    for linha in tabela {
        votos.push(Voto {
            secao: linha[0].parse().expect("secao ler_votacao"),
            cod_candidato: linha[1].parse().expect("cod_candidato ler_votacao"),
            numero_eleitor: linha[2].parse().expect("numero_eleitor ler_votacao"),
        });
    }
    votos
}
///Apresenta Estatística
fn menu_estatistica() {
    let candidatos = ["Jose", "Maria", "Brancos", "Nulos"];
    let resultado = colocacao();
    loop {
        println!(
            "
| --------------------------------- |
| MENU ESTATÍSTICA                  |
| --------------------------------- |
| 1 - Vencedor                      |
| 2 - Segundo colocado              |
| 3 - Quantidade de votos em branco |
| 4 - Quantidade de votos nulos     |
| 5 - Mostra eleitores              |
| 6 - Mostra apuração               |
| 9 - FIM                           |
| --------------------------------- |"
        );
        let opc = input_i32("Opção:");
        clear_console();
        match opc {
            1 => {
                if resultado[0] > resultado[1] {
                    println!("{} é o Vencedor.", candidatos[0])
                } else {
                    println!("{} é o Vencedor.", candidatos[1])
                }
            }
            2 => {
                if resultado[0] > resultado[1] {
                    println!("{} é o Segundo lugar.", candidatos[1])
                } else {
                    println!("{} é o Segundo lugar.", candidatos[0])
                }
            }
            3 => println!("Foram {} votos em Branco.", resultado[2]),
            4 => println!("Foram {} votos Nulos.", resultado[3]),
            5 => {
                println!("Mostrar Eleitore:");
                let eleitores = read_eleitores();
                for pessoa in eleitores {
                    pessoa.print();
                }
            }
            6 => {
                println!("Mostrar Agrupados:");
                let votos = ler_votacao("Apuração.md");
                for voto in votos {
                    voto.print();
                }
            }
            9 => break,
            _ => println!("Opção inválida"),
        };
    }
}
fn colocacao() -> [i32; 4] {
    let votos = ler_votacao("Apuração.md");
    let mut colocados = [0; 4];
    for voto in votos {
        colocados[(voto.cod_candidato - 1) as usize] += 1;
    }
    colocados
}
// Utils
struct Eleitor {
    numero_eleitor: i32,
    nome_eleitor: String,
    secao: i32,
}
impl Default for Eleitor {
    fn default() -> Self {
        Eleitor {
            numero_eleitor: 0,
            nome_eleitor: String::new(),
            secao: 0,
        }
    }
}
impl Eleitor {
    fn from_values(numero_eleitor: i32, nome_eleitor: String, secao: i32) -> Self {
        Eleitor {
            numero_eleitor,
            nome_eleitor,
            secao,
        }
    }
    fn print(&self) {
        println!(
            "[{},\"{}\",{}]",
            self.numero_eleitor, self.nome_eleitor, self.secao
        );
    }
}
struct Voto {
    secao: i32,
    cod_candidato: i32,
    numero_eleitor: i32,
}
impl Default for Voto {
    fn default() -> Self {
        Voto {
            secao: 0,
            cod_candidato: 0,
            numero_eleitor: 0,
        }
    }
}
impl Voto {
    fn from_values(secao: i32, cod_candidato: i32, numero_eleitor: i32) -> Self {
        Voto {
            secao,
            cod_candidato,
            numero_eleitor,
        }
    }
    fn print(&self) {
        println!(
            "[{},{},{}]",
            self.secao, self.cod_candidato, self.numero_eleitor
        );
    }
}

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
                continue;
            }
        };
        break;
    }
    integer
}
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
fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
