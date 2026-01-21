struct Node {
    next: Option<Box<Node>>,
    value: i32,
}

impl Node {
    fn new(value: i32, next: Option<Box<Node>>) -> Self {
        Self {
            next,
            value,
        }
    }
}

struct ListaLigada {
    head: Option<Box<Node>>,
    len: usize,
}

impl ListaLigada {
    fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    fn push(&mut self, value: i32) -> Result<bool, String> {
        let new: Box<Node> = Box::new(Node::new(value, self.head.take()));

        self.head = Some(new);

        Ok(true)
        
    }

    


}

fn main() {
    let mut lista_ligada: ListaLigada = ListaLigada::new();

    lista_ligada.push(32);

    
}
