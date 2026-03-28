#[derive(Debug, Clone)]
pub struct Patient {
    pub situation: u8,
    pub preferential: u8,
    pub time: u32
}

impl Patient {
    pub fn new(situation: u8, preferential: u8, time: u32) -> Self {
        Self {
            situation,
            preferential,
            time
        }
    }
}

pub type Queue = Vec<Patient>;