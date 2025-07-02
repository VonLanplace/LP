use std::{
    fmt::write,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, LineWriter, Write},
    vec,
};
fn main() {
    let mut notas = vec![];
    loop {
        println!(
            "
| ----------------- |
| Menu Principal    |
| ----------------- |
|1 – Carregar Notas |
|2 – Retirar Notas  |
|3 – Estatística    |
|9 – Fim            |
| ----------------- |"
        );
        let opc = input_i32("Opção:");
        match opc {
            1 => notas = carregar_notas(),
            2 => notas = retirar_notas(notas),
            3 => print_resumo(),
            9 => break,
            _ => println!("Opção inválida"),
        }
    }
}
fn carregar_notas() -> Vec<i32> {
    println!("Carregar Notas:");
    let mut notas = vec![100; 7];
    println!("{:?}", notas);
    notas
}
///Retirar notas
fn retirar_notas(mut notas: Vec<i32>) -> Vec<i32> {
    let mut resumo = vec![0.0; 5];
    resumo[0] = calc_saldo(&notas); //Valor inicial

    let mut i = 0;
    while i < 100 && calc_saldo(&notas) > 0.0 {
        let saldo = calc_saldo(&notas);
        println!("Saldo {}", calc_saldo(&notas));
        let valor = input_f64("Qual valor quer sacar?") as f64;
        println!("Valor {}", valor);
        if valor <= saldo && valor != 3.0 && valor != 1.0 {
            resumo[2] += valor; //Valor total Sacado
            notas = retirar_saque(notas, valor, 6);
            i += 1; //Contador de Saques
        } else if valor > saldo {
            println!("Valor inválido!");
            println!("Excedeu Saldo do Caixa!!");
        } else {
            println!("Valor inválido!");
            println!("Valor solicitado não pode ser sacado!!");
        }
    }
    resumo[3] = i as f64; //Quantidade de Saques
    resumo[1] = resumo[2] / resumo[3]; //Media dos Saques
    resumo[4] = resumo[0] - resumo[2]; //Valor das sobras do caixa
    gravar_resumo(resumo).expect("gravar_resumo em retirar_notas");
    notas
}
fn calc_saldo(notas: &Vec<i32>) -> f64 {
    let mut saldo = notas[0] * 2;
    saldo += notas[1] * 5;
    saldo += notas[2] * 10;
    saldo += notas[3] * 20;
    saldo += notas[4] * 50;
    saldo += notas[5] * 100;
    saldo += notas[6] * 200;
    saldo as f64
}
fn retirar_saque(mut notas: Vec<i32>, mut valor: f64, index: i32) -> Vec<i32> {
    let nota;
    if index >= 0 || valor > 0.0 {
        match index {
            0 => nota = 2.0,
            1 => nota = 5.0,
            2 => nota = 10.0,
            3 => nota = 20.0,
            4 => nota = 50.0,
            5 => nota = 100.0,
            6 => nota = 200.0,
            _ => nota = 0.0,
        }
        while valor >= nota && valor != nota + 1.0 && valor != nota + 3.00 {
            notas[index as usize] -= 1;
            valor -= nota;
        }
        retirar_saque(notas, valor, index - 1)
    } else {
        notas
    }
}
///gravar resumo
fn gravar_resumo(resumo: Vec<f64>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("Resumo.md")?;
    writeln!(
        file,
        "|Total inicial|Média dos saques|Valor dos saques|Quantidade de saques|Sobras do caixa|"
    )?;
    writeln!(file, "|-|-|-|-|-|")?;
    write!(file, "|")?;
    for casa in resumo {
        write!(file, "{}|", casa)?;
    }
    Ok(())
}
fn read_file(arquivo: &str) -> String {
    // This just reads the file into a String
    let content = fs::read_to_string(arquivo).expect("Erro de read_file");
    content
}
fn ler_resumo() -> io::Result<Vec<f64>> {
    let mut resumo = vec![0.0; 5];
    let mut monolito = read_file("Resumo.md");

    let mut matrix = [[""; 7]; 3];
    let mut i = 0;
    let mut j = 0;
    for lines in monolito.trim().split("\n") {
        for casas in lines.trim().split("|") {
            matrix[i][j] = casas;
            j += 1;
        }
        i += 1;
        j = 0;
    }
    for i in 0..resumo.len() {
        resumo[i] = matrix[2][i + 1].parse().expect("Erro parse ler_resumo!!");
    }
    Ok(resumo)
}
fn print_resumo() {
    let mut resumo = ler_resumo().expect("ler_resumo em print_resumo!");
    println!("O valor total inicial era {:.2}R$.", resumo[0]);
    println!("A média dos saques é {:.2}R$.", resumo[1]);
    println!("Valor total dos saques é {:.2}R$.", resumo[2]);
    println!("Quantidade de saques é {:.0}.", resumo[3]);
    println!("Valor total das sobras do caixa é {:.2}R$.", resumo[4]);
}
///Utils
fn input_i32(text: &str) -> i32 {
    let integer;
    loop {
        println!("{}", text);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("input_i32 linha 5");
        integer = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Valor inválido!!");
                continue;
            }
        };
        break;
    }
    integer
}
fn input_f64(text: &str) -> f64 {
    let pos_float;
    loop {
        let temp: u32;
        println!("{}", text);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("read_line in input_u64");
        temp = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Valor inválido!");
                continue;
            }
        };
        pos_float = temp as f64;
        break;
    }
    pos_float
}
