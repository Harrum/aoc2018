use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    println!("part 1: {}", part1().unwrap()); 

    println!("part 2: {}", part2().unwrap()); 

    Ok(())
}

fn part1() -> std::io::Result<i32> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);

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

    Ok(start)
}

fn part2() -> std::io::Result<i32> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);

    let mut start = 0;
    let mut freqs = Vec::new();
    let mut lines = Vec::new();

    for line in buf_reader.lines() {
        lines.push(line.unwrap());
    }
    

    let mut cont = true;
    let mut index = 0;

    while cont {
        freqs.push(start);

        if index >= lines.len() {
            index = 0
        }

        let line = lines.get(index).unwrap();
        let value = i32::from_str(line.get(1..).unwrap()).unwrap();

        if line.starts_with("+") {
            start += value;
        }
        else {
            start -= value;
        }
        index += 1;
        

        if freqs.contains(&start) {
            cont = false;
        }
    }
    
    Ok(start)
}