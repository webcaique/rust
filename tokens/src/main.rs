fn read_prompt() -> String {
    use std::io;

    let mut read: String = String::new();

    io::stdin().read_line(&mut read).expect("Erro ao ler");

    read
}

fn main() {
    let text = read_prompt();

    for word in text.trim().split(' ') {
        println!("{word}");
    }
}
