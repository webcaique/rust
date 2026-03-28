use crate::list::{Queue};

pub trait HeapSort {
    fn heap_sort_situation(&mut self);
    fn heap_sort_prefential(&mut self);
    fn heap_sort_time(&mut self);
}

impl HeapSort for Queue {
    fn heap_sort_situation(&mut self) {
        let mut heap_size: usize = self.len();
        let mut i: usize = heap_size - 1;
        build_heap_situation(self);

        loop {
            self.swap(0, i);
            heap_size -= 1;
            max_heap_situation(self, heap_size, 0);
            if i == 0 {
                break
            }
            i -= 1;
        }
    }

    fn heap_sort_prefential(&mut self) {
        let mut heap_size: usize = self.len();
        let mut i: usize = heap_size - 1;
        build_heap_prefential(self);

        loop {
            self.swap(0, i);
            heap_size -= 1;
            max_heap_preferential(self, heap_size, 0);
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    fn heap_sort_time(&mut self) {
        let mut heap_size: usize = self.len();
        let mut i: usize = heap_size - 1;

        build_heap_time(self);

        loop {
            self.swap(0, i);
            heap_size -= 1;
            max_heap_preferential(self, heap_size, 0);
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}

fn max_heap_situation(array: &mut Queue, heap_size: usize, i: usize) {
    let mut left: usize = 2 * i + 1;
    let mut right: usize = 2 * i + 2;
    let mut largest: usize = i;

    if left < heap_size && array[left].situation > array[largest].situation {
        largest = left;
    }
    if right < heap_size && array[right].situation > array[largest].situation {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        max_heap_situation(array, heap_size, largest);
    }
}

fn build_heap_situation(array: &mut Queue) {
    let heap_size: usize = array.len();
    let mut i: usize = heap_size;
    loop {
        max_heap_situation(array, heap_size, i);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    
}

fn max_heap_preferential(array: &mut Queue, heap_size: usize, i: usize) {
    let mut left: usize = 2 * i + 1;
    let mut right: usize = 2 * i + 2;

    let mut largest: usize = i;

    if left < heap_size && array[right].preferential > array[largest].preferential {
        largest = left;
    }
    if right < heap_size && array[right].preferential > array[largest].preferential {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        max_heap_preferential(array, heap_size, largest);
    }
}

fn build_heap_prefential(array: &mut Queue) {
    let heap_size: usize = array.len();
    let mut i: usize = heap_size;

    loop {
        max_heap_preferential(array, heap_size, i);
        if i == 0 {
            break;
        }
        i -= 1;
    }
}

fn max_heap_time(array: &mut Queue, heap_size: usize, i: usize) {
    let mut left: usize = 2 * i + 1;
    let mut right: usize = 2 * i + 2;
    let mut largest: usize = i;

    if left < heap_size && array[left].time > array[largest].time {
        largest = left;
    }

    if right < heap_size && array[right].time > array[largest].time {
        largest = right;
    }

    if largest != i {
        array.swap(i, largest);
        max_heap_time(array, heap_size, largest);
    }
}

fn build_heap_time(array: &mut Queue) {
    let heap_size: usize = array.len();
    let mut i: usize = heap_size;

    loop {
        max_heap_time(array, heap_size, i);

        if i == 0 {
            break;
        }

        i -= 1;
    }
}

