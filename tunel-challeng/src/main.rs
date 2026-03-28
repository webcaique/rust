fn read_prompt() -> String {
    use std::io;

    let mut read: String = String::new();

    io::stdin().read_line(&mut read).expect("Erro ao ler");

    read
}

fn parse_string_to_u32(_string: String) -> u32 {
    let number: u32 = _string.trim().parse().expect("Erro ao converter");

    number
}

fn read_number() -> u32 {
    parse_string_to_u32(read_prompt())
}

fn read_vector() -> Vec<u32> {
    let mut _vecU32: Vec<u32> = Vec::new();
    for value in read_prompt().split(' ') {
        _vecU32.push(parse_string_to_u32(value.to_string()));
    }

    _vecU32
}

fn read_matrix(rows: &usize) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for row in 0..(*rows) {
        matrix.push(read_vector());
    }

    matrix
}

fn volumes_calc(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    let mut _vecU32: Vec<u32> = vec![1; matrix.len()];
    let mut height: u32;

    for (key, list) in matrix.iter().enumerate() {
        height = *list.get(2).unwrap();
        if height < 41 {
            for value in list.iter(){
                _vecU32[key] *= value;
            }
        } else {
            _vecU32[key] = 0;
        }
    }

    _vecU32
}

fn main() {
    let number_of_boxes: usize = read_number() as usize;
    let matrix: Vec<Vec<u32>> = read_matrix(&number_of_boxes);

    let volumes_of_boxes: Vec<u32> = volumes_calc(matrix);

    for value in volumes_of_boxes.iter() {
        if(*value != 0) {
            println!("{}", value);
        }
    }
}
