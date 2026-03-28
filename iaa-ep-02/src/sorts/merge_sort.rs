use crate::list::{Queue};

pub trait MergeSort {
    fn merge_sort_situation(&mut self);
    fn merge_sort_preferential(&mut self);
    fn merge_sort_time(&mut self);
}

impl MergeSort for Queue {
    fn merge_sort_situation(&mut self) {
        let len: usize = self.len() - 1;
        merge_sort_situation_rec(self, 0, len);
    }

    fn merge_sort_preferential(&mut self) {
        let len: usize = self.len() -1;
        merge_sort_preferencial_rec(self, 0, len);
    }

    fn merge_sort_time(&mut self) {
        let len: usize = self.len() - 1;
        merge_sort_time_rec(self, 0, len);
    }
}

 
fn merge_sort_situation_rec(array: &mut Queue, init: usize, end: usize) {
    if init < end {
        let middle: usize = (init+end)/2;

        merge_sort_situation_rec(array, init, middle);
        merge_sort_situation_rec(array, middle +1, end);
        merge_situation(array, init, middle, end);
    }
}

fn merge_situation(array: &mut Queue, init: usize, middle: usize, end: usize) {
    let aux: Queue = array.clone();

    let mut i: usize = init;
    let mut j: usize = middle + 1;
    let mut k: usize = init;

    while i <= middle && j <= end {
        if aux[i].situation < aux[j].situation {
            array[k] = aux[i].clone();
            i += 1;
        } else {
            array[k] = aux[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i <= middle {
        array[k] = aux[i].clone();
        i += 1;
        k += 1;
    }

    while j <= end {
        array[k] = aux[j].clone();
        j += 1;
        k += 1;
    }
    
}

fn merge_sort_preferencial_rec(array: &mut Queue, init: usize, end: usize) {
    if init < end {
        let middle: usize = (init+end)/2;

        merge_sort_preferencial_rec(array, init, middle);
        merge_sort_preferencial_rec(array, middle + 1, end);
        merge_preferencial(array, init, middle, end);
    }
}

fn merge_preferencial(array: &mut Queue, init: usize, middle: usize, end: usize) {
    let aux: Queue = array.clone();

    let mut i: usize = init;
    let mut j: usize = middle + 1;
    let mut k: usize = init;

    while i <= middle && j <= end {
        if aux[i].preferential < aux[j].preferential {
            array[k] = aux[i].clone();
            i += 1;
        } else {
            array[k] = aux[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i <= middle {
        array[k] = aux[i].clone();
        i += 1;
        k += 1;
    }

    while j <= end {
        array[k] = aux[j].clone();
        j += 1;
        k += 1;
    }
}

fn merge_sort_time_rec(array: &mut Queue, init: usize, end: usize) {
    if init < end {
        let middle: usize = (init+end)/2;

        merge_sort_time_rec(array, init, middle);
        merge_sort_time_rec(array, middle + 1, end);
        merge_time(array, init, middle, end);
    }
}

fn merge_time(array: &mut Queue, init: usize, middle: usize, end: usize) {
    let aux: Queue = array.clone();

    let mut i: usize = init;
    let mut j: usize = middle + 1;
    let mut k: usize = init;

    while i <= middle && j <= end {
        if aux[i].time < aux[j].time {
            array[k] = aux[i].clone();
            i += 1;
        } else {
            array[k] = aux[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i <= middle {
        array[k] = aux[i].clone();
        i += 1;
        k += 1;
    }

    while j <= end {
        array[k] = aux[j].clone();
        j += 1;
        k += 1;
    }
}