fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    println!("👏 Seja bem vindo(a)! 👏 \n");
    loop {
        if start() {
            break;
        }
    }
}

fn start() -> bool {
    println!("Você deseja adicionar um novo TODO? (s/n)");

    let answer = input();
    if answer == "s" {
        println!("Qual o nome do arquivo TODO você deseja criar?");
        let name: String = input();
        println!("Seu novo arquivo TODO foi criado com sucesso.");
        println!("{}", name);
        false
    } else if answer == "n" {
        println!("Finalizando o programa.");
        true
    } else {
        println!("Insira uma entrada valida. Se deseja criar um novo TODO, insira 's', senão, insira 'n'.");
        false
    }
}
