fn start() {
    println!("👏 Seja bem vindo(a)! 👏\n");
    println!("Você deseja adicionar um novo TODO? (s/n)");
}

fn main() {
    start();

    fn input() -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
     }
        if let mut answer = 's' {
            println!("Qual o nome do arquivo TODO você deseja criar? \n");
            println!("Você digitou XXXXXXXXXX. Confirme o TODO a ser criado.\n");
            println!("O arquivo TODO foi criado com sucesso.\n");
            return start();
        }
        else if let answer = 'n' {
                println!("Finalizando o programa.");
            }
            else {
                println!("Insira uma entrada valida. Se deseja criar um novo TODO, insira 's', senão, insira 'n'.");

            }

        

}













    



