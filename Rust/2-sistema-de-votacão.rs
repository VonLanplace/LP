use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    clear_console();
    let mut votos = vec![];
    loop {
        println!(
            "
| --------------------------------- |
| SISTEMA DE VOTAÇÃO                |
| --------------------------------- |
| 1 – Carregar Seção/Número Eleitor |
| 2 – Classificar por Seção         |
| 3 – Gravar Registros              |
| 4 – Mostrar Indicadores           |
| 9 – Finalizar                     |
| --------------------------------- |"
        );
        let opc = input_i32();
        clear_console();
        match opc {
            1 => {
                println!("Carregar Votos:");
                votos = fcadastravotação();
            }
            2 => {
                println!("Classificar por Seção:");
                votos.sort_by_key(|x| x.numero_seção);
            }
            3 => {
                println!("Gravar registros:");
                write_voto(&votos).expect("Escrita de arquivo!");
            }
            4 => indicadores(),
            9 => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn fcadastravotação() -> Vec<Votacao> {
    let mut votos = vec![];
    for _i in 0..200 {
        let voto = Votacao::from_values(
            (simple_random() % 11) as i32,
            (simple_random() % 301) as i32,
        );
        votos.push(voto);
    }
    votos
}

///Indicadores
fn indicadores() {
    loop {
        println!(
            "
| ------------------------------------------------- |
| Mostrar Indicadores                               |
| ------------------------------------------------- |
| Estatísticas de Votação em 2021                   |
| 1 – Quantidade Eleitores por Seção                |
| 2 – Seção com Maior e Menor número de Eleitores   |
| 3 – Quantidade de votos por candidato             |
| 4 – 10 primeiros colocadas (nro cand. e qtd votos)|
| 9 – Finaliza consulta                             |
| ------------------------------------------------- |"
        );
        let opc = input_i32();
        clear_console();
        match opc {
            1 => eleitores_por_secao(),
            2 => secao_maior_menor(),
            3 => votos_por_candidato(),
            4 => primeiros_colocados(),
            9 => break,
            _ => println!("Opção inválida!"),
        }
    }
}
fn eleitores_por_secao() {
    println!("Eleitores por secao:");
    let mut votos = read_voto();
    votos.sort_by_key(|z| z.numero_seção);
    let mut secao = [0; 11];
    for vot in votos {
        secao[vot.numero_seção as usize] += 1;
    }
    for i in 0..secao.len() {
        println!("A seção {:2.} tem {:3.} eleitores;", i + 1, secao[i]);
    }
}
fn secao_maior_menor() {
    let mut votos = read_voto();
    votos.sort_by_key(|z| z.numero_seção);
    let mut secao = [0; 11];
    for vot in votos {
        secao[vot.numero_seção as usize] += 1;
    }
    {
        let mut maior = 0;
        let mut menor = 0;
        for i in 0..secao.len() {
            if secao[i] > secao[maior] {
                maior = i;
            } else if secao[i] < secao[menor] {
                menor = i;
            }
        }
        println!(
            "A seção com maior numero de eleitores é a {} com {} eleitores.",
            maior, secao[maior]
        );
        println!(
            "A seção com menor numero de eleitores é a {} com {} eleitores.",
            menor, secao[menor]
        )
    }
}
fn read_candidatos() -> Vec<Votacao> {
    let mut votos = read_voto();
    votos.sort_by_key(|z| z.numero_candidato);
    let mut candidatos = vec![];

    // candidatos ler
    candidatos.push(Votacao::from_values(0, votos[0].numero_candidato));
    for voto in votos {
        let size = ((candidatos.len() as i32) - 1) as usize;
        if voto.numero_candidato > candidatos[size].numero_candidato {
            candidatos.push(Votacao::from_values(1, voto.numero_candidato));
        } else {
            candidatos[size].numero_seção += 1;
        }
    }
    candidatos
}
fn votos_por_candidato() {
    println!("Votos por Candidato:");
    let candidatos = read_candidatos();
    for candidado in candidatos {
        println!(
            "O candidato {:3.} recebeu {:3.} votos.",
            candidado.numero_candidato, candidado.numero_seção
        );
    }
}
fn primeiros_colocados() {
    println!("Colocações:");
    let mut candidatos = read_candidatos();
    candidatos.sort_by_key(|x| std::cmp::Reverse(x.numero_seção));

    for i in 0..10 {
        println!(
            "{:2.}° lugar candidato {}.",
            i + 1,
            candidatos[i].numero_candidato
        );
    }
}
/// Votacao "Class"
struct Votacao {
    numero_seção: i32,
    numero_candidato: i32,
}

impl Votacao {
    fn from_values(numero_seção: i32, numero_candidato: i32) -> Self {
        Votacao {
            numero_seção,
            numero_candidato,
        }
    }
}

impl Default for Votacao {
    fn default() -> Self {
        Votacao {
            numero_seção: 0,
            numero_candidato: 0,
        }
    }
}

fn write_voto(votos: &Vec<Votacao>) -> std::io::Result<()> {
    //It has to have error corection
    let mut file = OpenOptions::new() // The optons for file creation are they
        // necessary??
        .write(true) //write permission
        .truncate(true) //clear the file
        .create(true) //makes the file
        .open("Votação.md")?; //what to open

    writeln!(file, "|numero_seção|numero_candidato|\n|-|-|")?; //The usual .md header
    for vot in votos {
        // Simple all loop
        writeln!(file, "|{}|{}|", vot.numero_seção, vot.numero_candidato)?; //This writes line for
        //line
    }
    Ok(()) // It has to return with the capacity or pushing errors up
}

fn read_file(arquivo: &str) -> io::Result<String> {
    // This just reads the file into a String
    let content = fs::read_to_string(arquivo)?;
    Ok(content)
}
fn read_voto() -> Vec<Votacao> {
    //The unholy convertion from a String to Votação
    let content = read_file("Votação.md").expect("Erro de Leitura Votação.md"); //Pulls
    //string from file reader function
    let mut votos = vec![]; //This is our output
    for line in content.split("\n") {
        //So first it breaks the blok into lines
        let casa: Vec<&str> = line.split("|").collect(); //Now it breaks lines intos casa 
        if casa.len() > 1 {
            // Test if there is content
            votos.push(Votacao::from_values(
                //Using the builder implementation
                match casa[1].trim().parse() {
                    //this tests if it can be parsed
                    Ok(n) => n,         // no error so match Ok - then value n
                    Err(_) => continue, // error so match Err - then continue
                },
                match casa[2].trim().parse() {
                    //this tests if it can be parsed
                    Ok(n) => n,         // no error so match Ok - then value n
                    Err(_) => continue, // error so match Err - then continue
                },
            ));
        }
    }
    votos //Return votos
}

fn input_i32() -> i32 {
    let integer: i32;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro de stdin.read_line!!");
        integer = match input.trim().parse() {
            Ok(n) => {
                if n > 0 {
                    n
                } else {
                    println!("Valor inválido!");
                    continue;
                }
            }
            Err(_) => {
                println!("Valor inválido!");
                continue;
            }
        };
        break;
    }
    integer
}

fn simple_random() -> u32 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32; // Convert time to a u32 seed

    // A very basic pseudo-random generator (LCG)
    let mut rng = seed;
    rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
    rng
}
