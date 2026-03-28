mod read_file;
mod list;
mod sorts;

use crate::read_file::{read};
use crate::list::{Queue};
use crate::sorts::insert_sort::InsertSort;
use crate::sorts::merge_sort::MergeSort;
use crate::sorts::quick_sort::QuickSort;
use crate::sorts::heap_sort::HeapSort;

fn print(array: &Queue) {
    for row in array {
        println!("{:?}", row);
    }
}

fn main() {
    let mut patient_queue: Queue = read(String::from("src/dados.csv")).expect("Erro ao ler o arquivo");
    patient_queue.merge_sort_time();
    patient_queue.quick_sort_preferential();
    patient_queue.merge_sort_situation();
    print(&patient_queue);
    ()
}
