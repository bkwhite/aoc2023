use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/input.txt")?;
    let reader = BufReader::new(file);
    let cube_counts: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut possible_games = 0;

    for line in reader.lines() {
        let line_text = line.unwrap();
        let linep = line_text.as_str();
        let re = Regex::new(r"(\d+)").unwrap();
        let Some(caps) = re.captures(linep) else {
            return Ok(());
        };
        let game_num: u32 = caps[0].parse().unwrap();

        let game_str = format!("Game {}:", game_num);
        let replace_me = game_str.as_str();
        let game_str_rp = linep.replace(replace_me, "");
        let sets: Vec<&str> = game_str_rp.split(";").collect();

        let mut game_cube_counts: HashMap<&str, u32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for set in sets {
            let cube_counts_raw: Vec<&str> = set.trim().split(",").map(|s| s.trim()).collect();

            for raw_cube_count in cube_counts_raw {
                let cc: Vec<&str> = raw_cube_count.split(" ").collect();
                let count: u32 = cc[0].parse().unwrap();
                let color = cc[1];

                if game_cube_counts[color] < count {
                    *game_cube_counts.get_mut(color).unwrap() = count;
                }
            }
        }

        println!("{:?}", game_cube_counts);

        if &game_cube_counts["red"] <= &cube_counts["red"]
            && &game_cube_counts["green"] <= &cube_counts["green"]
            && &game_cube_counts["blue"] <= &cube_counts["blue"]
        {
            println!("adding possible {}", game_num);
            possible_games += game_num;
            println!("possible {}", possible_games);
        }
    }

    println!("possible games {}", possible_games);
    Ok(())
}
