use std::io::{Stdin, Stdout, Write};

fn main() {
    println!("👏 Seja bem vindo(a)!!! 👏 \n");
    let mut terminal = Terminal::new();
    loop {
        if terminal.start() {
            let message = terminal.ask_for_new_todo();
            terminal.show_todo(&message);
        } else {
            std::process::exit(0);
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
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        println!("Qual o nome do arquivo TODO você deseja criar?");
        let todo = self.input();

        Todo::new(todo)
    }

    fn show_todo(&mut self, todo: &Todo) {
        println!(self.stdout, "Todo criado: {}", todo.message).unwrap();
    }
    fn start(&mut self) -> bool {
        loop {
            println!("Você deseja adicionar um novo TODO? (s/n)");
            let answer = self.input();
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
    fn input(&mut self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}
