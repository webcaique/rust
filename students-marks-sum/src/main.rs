fn read_prompt() -> String {
use std::io;

    let mut read: String = String::new();

    io::stdin().read_line(&mut read).expect("Erro ao ler");

    read
}

fn parse_to_int(str_num: String) -> u32 {
    let num: u32 = str_num.trim().parse().expect("Erro ao ler!");

    num
}

fn read_number() -> u32 {
    parse_to_int(read_prompt())
}

fn read_vector(_size: &usize) -> Vec<u32> {
    let mut _vector: Vec<u32> = Vec::new();
    for i in 0..(*_size) {
        _vector.push(read_number());
    }

    _vector
}

fn sum_vector(vec: Vec<u32>, init: u8) -> u32 {
    let mut _sum: u32 = 0;
    for value in vec.iter().skip(init as usize).step_by(2) {
        _sum += value;
    }

    _sum
}

fn sum_marks(_vec: Vec<u32>, gender: char) -> u32{
    match gender {
        'b' => sum_vector(_vec, 0),
        'g' => sum_vector(_vec, 1),
        _ => { panic!("Erro inesperado!");}
    }
}

fn main() {
    let _size: usize = read_number() as usize;

    let vector: Vec<u32> = read_vector(&_size);

    let _type: char = match read_prompt().as_str().trim() {
        "b" => 'b',
        "g" => 'g',
        _ => { panic!("Erro ao digitar o gênero (b/g)") },
    };

    let _sum:u32 = sum_marks(vector, _type);

    println!("{_sum}");
}
