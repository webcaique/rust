//--- CONTANTES ---//
const MOVS: [[i32; 2]; 4] = [
    [1, 0],  // direita
    [-1, 0], // esquerda
    [0, 1],  // baixo
    [0, -1], // cima
];

//--- TYPES ---//
type Row = Vec<u32>;
type Matrix = Vec<Row>;

//--- STRUCTS ---//
struct Island {
    sum_of_insland: u32,
    next: Option<Box<Island>>
}

struct ListIsland {
    head: Option<Box<Island>>,
    len: usize
}

//--- IMPL STRUCTS ---//
impl Island {
    fn new(next: Option<Box<Island>>) -> Self {
        Self {
            sum_of_insland: 0,
            next
        }
    }

    fn update_value(&mut self, value: u32) {
        self.sum_of_insland += value;
    }

    fn update_next(&mut self, next: Option<Box<Island>>) {
        self.next = next;
    }

}


impl ListIsland {
    fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    fn push(&mut self, mut new: Box<Island>) {
        new.update_next(self.head.take());

        self.head = Some(new);
    }

    fn update_len(&mut self) {
        self.len = self.len + 1;
    }

    fn print(&self) {
        let mut current: &Option<Box<Island>> = &self.head;

        while let Some(island) = current {
            print!("{} ", island.sum_of_insland);
            current = &island.next;
        }
        print!("\n");
    }

}

//--- MAIN ---//
fn main() {
    use std::env;

    let mut args = env::args();

    let _ = args.next();



    let total_depth: u32 = args.next().expect("Número não informado").parse().expect("Número de profundidade total inválido");
    let rows: usize = args.next().expect("Número não informado").parse().expect("Número de linhas inválido");
    let colls: usize = args.next().expect("Número não informado").parse().expect("Número de colunas inválidos");
    let file_name: String = args.next().expect("Nome do arquivo não informado");

    let matrix: Matrix = read_file(file_name, rows, colls);

    let lista: ListIsland = count_insland(matrix);

    println!("{}", lista.len);
    lista.print();

    let vec: Option<Vec<u32>> = total_depth_sum(lista, total_depth);

    if let Some(_vec) = vec {
        println!("{:?}", _vec);
    } else {
        println!("Não encontrado a sequência máxima!");
    }

}

//--- FUNCTIONS ---//
fn read_file(file_name: String, n_rows: usize, n_colls: usize) -> Matrix {
    use std::fs;

    let content:String = fs::read_to_string(file_name).expect("Erro ao ler o arquivo!");

    let mut matrix: Matrix = Vec::with_capacity(n_rows);
    let mut row: Row;
    for _ in 0..n_rows {
        row = Vec::with_capacity (n_colls);
        matrix.push(row);
    }
    matrix = content
    .lines()
    .map(|linha| {
        linha
            .split_whitespace()
            .map(|num| {
                num.parse::<u32>().unwrap()
            })
            .collect()
    })
    .collect();
    
        
    matrix
}

fn discovery(matrix: &mut Matrix, island: &mut Island, x: i32, y: i32) {
    if y < 0 || x < 0 {
        return;
    }

    let (y, x) = (y as usize, x as usize);

    if y >= matrix.len() || x >= matrix[y].len() {
        return;
    }

    let value = matrix[y][x];
    if value == 0 {
        return;
    }

    island.update_value(value);
    matrix[y][x] = 0;

    for mov in MOVS {
        discovery(matrix, island, x as i32 + mov[0], y as i32 + mov[1]);
    }
}


fn count_insland(mut matrix: Matrix) -> ListIsland {
    let mut list = ListIsland::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x] > 0 {
                let mut island = Box::new(Island::new(None));
                discovery(&mut matrix, &mut island, x as i32, y as i32);
                list.push(island);
                list.update_len();
            }
        }
    }

    list
}

fn total_depth_sum_rec
    (island: &Option<Box<Island>>,
    vector: &mut Vec<u32>,
    total_sum: u32,
    total_depth: u32,
    init: usize) -> bool {

    if total_sum == total_depth {
        return true;
    }
    if total_sum > total_depth {
        return false;
    }
    let mut next_island: &Option<Box<Island>> = island;
    for i in init..(vector.capacity()) {
        if let Some(_island) = next_island {
            next_island = &_island.next;
            let q: bool = 
            total_depth_sum_rec(next_island, 
                vector, total_sum + _island.sum_of_insland, 
                total_depth, 
                i);
            if q {
                vector.push(_island.sum_of_insland);
                return true;
            }
        }
    }

    false
}

fn total_depth_sum(lista: ListIsland, total_depth: u32) -> Option<Vec<u32>> {
    let mut vec: Vec<u32> = Vec::with_capacity(lista.len);

    if total_depth_sum_rec(&lista.head, &mut vec, 0, total_depth, 0) {
        return Some(vec);
    }

    None
}