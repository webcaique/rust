use crate::list::{Patient, Queue};

pub fn read(file_name: String) -> Result<Queue, Box<dyn std::error::Error>> {
   
    let mut reader = csv::Reader::from_path(file_name)?;
    let mut patient_queue: Queue = vec![];

    for line in reader.records() {
        let record = line?;
        let time: u32 = record[2].parse::<u32>().expect("Erro ao converter");
        let situation: u8 = match &record[0] {
            "Vermelho" => 4,
            "Laranja" => { if time > 10 { 4 } else { 3 } },
            "Amarelo" => { if time > 60 { 3 } else { 2 } },
            "Verde" => { if time > 120 { 2 } else { 1 } },
            "Azul" => { if time > 240 { 1 } else { 0 } },
            _ => panic!("Erro na situation")
        };

        let preferential: u8 = match &record[1] {
            "N/A" => 0,
            _ => 1
        };

        patient_queue.push(Patient::new(situation, preferential, time));
    }

    Ok(patient_queue)
}