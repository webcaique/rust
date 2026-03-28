mod garden;

use garden::vegetables::*;

fn main() {
    let test: VegetablesInfo = VegetablesInfo::new("28/10/2025".to_string(), "Chuchu".to_string());
    if test.due_date_valid() {
        println!("Vencido!");
    }
}
