use crate::list::{Patient, Queue};

pub trait InsertSort {
    fn insert_sort_situation(&mut self);
    fn insert_sort_preferential(&mut self);
    fn insert_sort_time(&mut self);


}

impl InsertSort for Queue {
    fn insert_sort_situation(&mut self) {
        let n: usize = self.len();

        for i in 1..n {
            let key: Patient = self[i].clone();
            let mut j: usize = i;

            while j > 0 && self[j-1].situation > key.situation {
                self[j] = self[j-1].clone();
                j -= 1;
            }

            self[j] = key;
        }
    }

    fn insert_sort_preferential(&mut self) {
        let n: usize = self.len();

        for i in 1..n {
            let key: Patient = self[i].clone();
            let mut j: usize = i;

            while j > 0 && self[j-1].preferential > key.preferential {
                self[j] = self[j-1].clone();
                j -= 1;
            }

            self[j] = key;
        }
    }

    fn insert_sort_time(&mut self) {
        let n: usize = self.len();

        for i in 1..n {
            let key: Patient = self[i].clone();
            let mut j: usize = i;

            while j > 0 && self[j-1].time > key.time {
                self[j] = self[j-1].clone();
                j -= 1;
            }

            self[j] = key;
        }
    }
}
