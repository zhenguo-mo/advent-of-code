use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Part 1

// fn main() -> io::Result<()> {
//     let file = File::open("src/puzzle.txt")?;
//     let reader = BufReader::new(file);
    
//     let mut horizontal = 0;
//     let mut depth = 0;

//     for line in reader.lines() {
//         let line = line?;
//         let mut split = line.split(' ');
//         let command = split.next().unwrap();
//         let value: i32 = split.next().unwrap().parse().unwrap();

//         match command {
//             "forward" => { horizontal += value; }
//             "down"    => { depth += value; }
//             "up"      => { depth -= value; }
//             _         => println!("Oops")
//         }

//     }
//     println!("Horizontal: {horizontal}");
//     println!("Depth: {depth}");
//     println!("Answer: {}", horizontal * depth);

//     Ok(())
// }
// -------------------------------------------------------------------------------------

// Part 2

fn main() -> io::Result<()> {
    let file = File::open("src/puzzle.txt")?;
    let reader = BufReader::new(file);
    
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let value: i32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => { 
                horizontal += value;
                depth += aim * value;
            }
            "down"    => { aim += value; }
            "up"      => { aim -= value; }
            _         => println!("Oops")
        }

    }
    println!("Horizontal: {horizontal}");
    println!("Depth: {depth}");
    println!("Answer: {}", horizontal * depth);

    Ok(())
}
