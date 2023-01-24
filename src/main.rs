use std::io::{Stdin, Stdout, Write};

fn main() {
    println!("👏 Seja bem vindo(a)! 👏 \n");
    loop {
        let mut terminal = Terminal::new();
        if !terminal.start() {
            std::process::exit(0);
        } else {
            let mut message = terminal.ask_for_new_todo();
            terminal.show_todo(&message);
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    pub fn new(message: String) -> Self {
        Todo { message }
    }
}
struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            stdin: (std::io::stdin()),
            stdout: (std::io::stdout()),
        }
    }
    fn ask_for_new_todo(&mut self) -> Todo {
        println!("Qual o nome do arquivo TODO você deseja criar?");
        let todo = input();

        Todo::new(todo)
        // Faça a sua lógica para ler a mensagem que será utilizada para construir o
        // `Todo`.
        //
        // Para implementar essa função, você poderá aproveitar parte da lógica da
        // função `input` que apresentamos no projeto anterior. Porém, ao invés de
        // chamar `std::io::stdin()` para invocar o método `.read_line()`, você poderá
        // usar a instância de `Stdin` que já está no próprio terminal. Para isso, você
        // pode fazer `self.stdin.read_line(...)`.
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "Todo criado: {}", todo.message).unwrap();
    }
    fn start(&mut self) -> bool {
        loop {
            println!("Você deseja adicionar um novo TODO? (s/n)");
            let answer = input();
            if answer == "s" {
                return true;
            } else if answer == "n" {
                println!("Finalizando o programa.");
                return false;
            } else {
                println!("Insira uma entrada valida. Se deseja criar um novo TODO, insira 's', senão, insira 'n'.");
            }
        }
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
