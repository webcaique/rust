use super::vegetables;

pub mod spices {
    fn print_any() {
        let corinthias: VegetablesInfo = VegetablesInfo::new("28/10/2025".to_string(), "Chuchu".to_string());
        println!("{}", corinthias.due_date);
    }
}