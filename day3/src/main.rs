use std::fs::File;
use std::io:: {self, prelude::*, BufReader};
// Only supports up to 32 bits
struct BinNumber {
    length: usize,
    value: u32
}

impl BinNumber {
    fn new(number: &String) -> Self {
        Self {
            length: number.len(),
            value: u32::from_str_radix(number, 2).unwrap()
        }
    }

    fn digit_at(&self, n: u8) -> u32 {
        (self.value >> n) & 1
    }
}

fn calc_gamma_rate(bit_counts: Vec<u32>, total_count: u32) -> u32 {
    let gamma_rate: String = bit_counts.into_iter().map(|count| (count / (total_count/2)).to_string()).rev().collect();
    u32::from_str_radix(&gamma_rate, 2).unwrap()
}

fn flip_n_bits(num: u32, n: u8) -> u32 {
    num ^ ((1u32 << n) - 1)
}

fn main() -> io::Result<()> {
    let file = File::open("src/puzzle.txt")?;
    let reader = BufReader::new(file);
    let mut num;
    let mut line_length = 0;
    let mut total_lines = 0;
    let mut counts: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        total_lines += 1;
        num = BinNumber::new(&line);
        if counts.is_empty() {
            counts = vec![0; num.length];
            line_length = line.len();
        }

        for bit_index in 0 .. num.length {
            counts[bit_index] += num.digit_at(bit_index as u8);
        }
    }

    let gamma = calc_gamma_rate(counts, total_lines);
    let epsilon = flip_n_bits(gamma, line_length as u8);
    let power_consumption = gamma * epsilon;
    println!("Gamma: {gamma}");
    println!("Epsilon: {epsilon}");
    println!("Power consumption: {power_consumption}");
    Ok(())
}
