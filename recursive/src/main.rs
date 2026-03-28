fn read_prompt() -> String {
    use std::io;

    let mut read: String = String::new();

    io::stdin().read_line(&mut read).expect("Erro ao ler");

    read
}

fn parse_string_to_u32(str_num: String) -> u32 {
    let num: u32 = str_num.trim().parse().expect("Erro ao ler");

    num
}

fn read_number() -> u32 {
    parse_string_to_u32(read_prompt())
}

fn sum_S(n: u32, a: u32, b: u32, c: u32) -> u32 {
    if n == 1 { return a; }
    if n == 2 { return b; }
    if n == 3 { return c; }

    sum_S(n-1, a, b, c) + sum_S(n-2, a, b, c) + sum_S(n-3, a, b, c)

}

fn init_sum_S(n: u32) -> u32 {
    let frase: String = read_prompt();
    let mut _vector: Vec<u32> = Vec::new();

    for (key, value) in frase.trim().split(' ').enumerate() {
        _vector.push(parse_string_to_u32(value.to_string()));
    }

    sum_S(n, _vector[0], _vector[1], _vector[2])
}


fn main() {
    let enesimo: u32 = read_number();

    println!("{}", init_sum_S(enesimo));

}
