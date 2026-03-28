fn sum_of_digit(num: &u32) -> u32 {
    if num/10 == 0  { return num%10; }
    let res: u32 = num%10;
    let quo: u32 = num/10;
    
    res + sum_of_digit(&quo)
}

fn main() {
    let number: u32 = 243;
    let result: u32 = sum_of_digit(&number);

    println!("Resultado: {result}.");
}
