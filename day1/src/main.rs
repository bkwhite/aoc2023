use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_sum = 0;

    let number_strings: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for line in reader.lines() {
        let original_line = line.unwrap();
        let first = get_first(original_line.clone(), &number_strings);
        let last = get_last(original_line.clone(), &number_strings);
        let num: u32 = format!("{}{}", first, last).parse().unwrap();
        total_sum += num;
    }

    println!("{:?}", total_sum);
    Ok(())
}

fn get_first(input: String, number_strings: &HashMap<&str, &str>) -> String {
    let mut temp_str = String::from("");
    let re = Regex::new(r"(\d{1})").unwrap();
    let mut first = String::from("");

    for c in input.chars() {
        if re.captures_iter(c.to_string().as_str()).count() == 0 {
            temp_str.push(c);
        } else {
            first = c.to_string();
            break;
        }

        for key in number_strings.keys() {
            if temp_str.as_str().contains(key) {
                first = number_strings.get(key).unwrap().to_string();
                break;
            }
        }

        if first != "" {
            break;
        };
    }

    return first;
}

fn get_last(input: String, number_strings: &HashMap<&str, &str>) -> String {
    let mut temp_str = String::from("");
    let re = Regex::new(r"(\d{1})").unwrap();
    let mut last = String::from("");

    for c in input.chars().rev() {
        if re.captures_iter(c.to_string().as_str()).count() == 0 {
            temp_str.push(c);
        } else {
            last = c.to_string();
            break;
        }

        let backwards = String::from_iter((&*temp_str).chars().rev());

        for key in number_strings.keys() {
            if backwards.as_str().contains(key) {
                last = number_strings.get(key).unwrap().to_string();
                break;
            }
        }

        if last != "" {
            break;
        };
    }

    return last;
}
