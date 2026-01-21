fn main() -> std::io::Result<()> {
    use rand::Rng;
    use std::fs::File;
    use std::io::{Write, BufWriter};
    use std::env;

    let mut args = env::args();

    let _ = args.next();



    let file = File::create("../iaa-ep-01/src/matrix.txt")?;
    let mut buffer = BufWriter::new(file);

    let ROWS: usize = args.next().expect("Linhas não informadas").parse().expect("Erro na conversão (colunas)");
    let COLLS: usize = args.next().expect("Colunas não informadas").parse().expect("Erro na conversão (linhas)");

    let ZERO: u8 = args.next().expect("Peso do zero não informado").parse().expect("Erro ao converter (peso do zero)");
    let mut rng = rand::thread_rng();
    let mut weigth: u8;
    let mut num: u32;
    let mut xadrez: usize = 0;

    for _i in 0..ROWS {
        for c in 0..COLLS {
            weigth = rng.gen_range(1..=10);
            if weigth <= ZERO {
                num = 0;
            } else {
                num = rng.gen_range(0..(256));
            }
            if c > 0 {
                write!(buffer, " ")?;
            }
            write!(buffer, "{}", num)?;
        }
        let _ = writeln!(buffer);
    }
    // for _i in 0..ROWS {
    //     for c in 0..COLLS {
    //         if (c+xadrez)%2 == 0 {
    //             num = 1;
    //         } else {
    //             num = 0;
    //         }
    //         if c > 0 {
    //             write!(buffer, " ")?;
    //         }
    //         write!(buffer, "{}", num)?;
    //     }
    //     let _ = writeln!(buffer);
    //     xadrez = (xadrez+1)%2;
    // }



    Ok(())
}
