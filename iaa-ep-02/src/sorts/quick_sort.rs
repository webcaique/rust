use crate::list::{Patient, Queue};

pub trait QuickSort {
    fn quick_sort_situation(&mut self);
    fn quick_sort_preferential(&mut self);
    fn quick_sort_time(&mut self);
}

impl QuickSort for Queue {
    fn quick_sort_situation(&mut self) {
        let len: usize = self.len() - 1;
        quick_situation(self, 0 , len);
    }

    fn quick_sort_preferential(&mut self) {
        let len: usize = self.len() - 1;
        quick_preferential(self, 0 , len);
    }

    fn quick_sort_time(&mut self) {
        let len: usize = self.len() - 1;
        quick_time(self, 0 , len);
    }
}

fn particao_situation(array: &mut Queue, left: usize, right: usize) -> usize{
    let meio: usize = (left+right)/2;
    let pivo: Patient = array[meio].clone();
    let mut i: usize = left;
    let mut j: usize = right;

    loop {
        while array[i].situation < pivo.situation {
            i += 1;
        }

        while array[j].situation > pivo.situation {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        array.swap(i, j);

        i += 1;
        j -= 1;

    }


}

fn quick_situation(array: &mut Queue, left: usize, right: usize) {
    if left < right {
        let pivo: usize = particao_situation(array, left, right);
        if pivo > 0 {
            quick_situation(array, left, pivo);
        }
        quick_situation(array, pivo+1, right);
    }
}

fn particao_preferential(array: &mut Queue, left: usize, right: usize) -> usize {
    let meio: usize = (left+right)/2;
    let pivo: Patient = array[meio].clone();
    let mut i: usize = left;
    let mut j: usize = right;

    loop {
        while array[i].preferential < pivo.preferential {
            i += 1;
        }

        while array[j].preferential > pivo.preferential {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        array.swap(i, j);

        i += 1;
        j -= 1;
    }
}

fn quick_preferential(array: &mut Queue, left: usize, right: usize) {
    if left < right {
        let pivo: usize = particao_preferential(array, left, right);
        if pivo > 0 {
            quick_preferential(array, left, pivo);
        }
        quick_preferential(array, pivo+1, right);
    }
}

fn particao_time(array: &mut Queue, left: usize, right: usize) -> usize {
    let meio: usize = (left+right)/2;
    let pivo: Patient = array[meio].clone();

    let mut i: usize = left;
    let mut j: usize = right;

    loop {
        while array[i].time < pivo.time {
            i += 1;
        }

        while array[j].time > pivo.time {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        array.swap(i,j);

        i += 1;
        j -= 1;
    }
}

fn quick_time(array: &mut Queue, left: usize, right: usize) {
    if left < right {
        let pivo: usize = particao_time(array, left, right);
        if pivo > 0 {
            quick_situation(array, left, pivo);
        }
        quick_situation(array, pivo +1, right);
    }
}