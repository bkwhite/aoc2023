use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let re = Regex::new(r"(\d{1})").unwrap();
        let mut numbers_in_line: Vec<u32> = Vec::new();

        for cap in re.captures_iter(&line) {
            let num: u32 = cap[1].parse().unwrap();
            numbers_in_line.push(num);
        }

        if numbers_in_line.len() > 0 {
            let first = numbers_in_line.first().unwrap();
            let last = numbers_in_line.last().unwrap();

            println!("{:?} {:?} | line: {}", first, last, index);

            let num: u32 = format!("{}{}", first, last).parse().unwrap();
            total_sum += num;
        }
    }

    println!("{:?}", total_sum);
    Ok(())
}
