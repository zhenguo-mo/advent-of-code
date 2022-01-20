use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/puzzle.txt")?;
    let reader = BufReader::new(file);
    
    let mut answer = 0;
    let mut window: Vec<i32> = Vec::new();
    let mut current_total = 0;
    let mut index = 0;

    for line in reader.lines() {
        let number: i32 = line?.parse().unwrap();
        if window.len() < 3 {
            window.push(number);
            current_total += number;
        } else {
            let next_total = current_total - window[index] + number;
            if next_total > current_total {
                answer += 1;
            }
            current_total = next_total;
            window[index] = number;
            index = (index + 1) % 3;
        }
    }


    println!("{}", answer);

    Ok(())
}
