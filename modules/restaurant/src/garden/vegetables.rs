mod spices;


enum Vegetables {
    Abobrinha,
    Chuchu,
    Berinjela,
    Couve
}

pub struct VegetablesInfo {
    due_date: String,
    vegetable: Vegetables,
}

impl VegetablesInfo {
    pub fn new(due_date: String, _vegetable_string: String) -> Self{
        let vegetable: Vegetables = match _vegetable_string.as_str() {
            "Abobrinha" => Vegetables::Abobrinha,
            "Chuchu" => Vegetables::Chuchu,
            "Berinjela" => Vegetables::Berinjela,
            "Couve" => Vegetables::Couve,
            _ => { panic!("Erro inesperado!"); }
        };
        print_any();
        Self {
            due_date,
            vegetable
        }
    }

    pub fn due_date_valid(&self) -> bool {
        let date: Vec<&str> = self.due_date.trim().split('/').collect();
        let diff:u32 = date[2].parse().expect("Error bluh");
        (2025 - diff ) <= 0
    }
}