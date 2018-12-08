use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    let mut start = 0;

    for line in buf_reader.lines() {
        let l = line.unwrap();
        
        let value = i32::from_str(l.get(1..).unwrap()).unwrap();

        if l.starts_with("+") {
            start += value;
        }
        else {
            start -= value;
        }
    }

    println!("{}", start);

    Ok(())
}