use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


pub fn run() -> String {
    let input = read_file().unwrap();
    format!("part one: {:?} | part two: {:?}", compute_part_one(&input), compute_part_two(&input))
}

fn read_file() ->  std::io::Result<(String)> {   
    let mut file = File::open("src/day_one/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn compute_part_one(input: &String) -> i32 {
    let mut frequency = 0;
    for line in input.lines() {
        frequency += line.parse::<i32>().unwrap();
    }

    frequency
}

fn compute_part_two(input: &String) -> i32 {
    let mut current_frequency = 0;
    let mut freq_seen_twice = false;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(current_frequency);
    
    while freq_seen_twice == false {
        for line in input.lines() {
            current_frequency += line.parse::<i32>().unwrap();
            
            if seen_frequencies.contains(&current_frequency) {
                freq_seen_twice = true;
                break;
            }
            
            seen_frequencies.insert(current_frequency);
        }
    }

    current_frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads() {
        let contents = read_file();
        assert_eq!(true, contents.is_ok())
    }

    #[test]
    fn it_computes() {
        let contents = compute_part_one(&"+1\r\n-2\r\n+3\r\n".to_string());
        assert_eq!(2, contents)
    }

    #[test]
    fn it_computes_part_two() {
        let output = compute_part_two(&"+1\r\n-1\r\n".to_string());
        assert_eq!(0, output);

        let output2 = compute_part_two(&"+3\r\n+3\r\n+4\r\n-2\r\n-4".to_string());
        assert_eq!(10, output2);

        let output3 = compute_part_two(&"-6\r\n+3\r\n+8\r\n+5\r\n-6".to_string());
        assert_eq!(5, output3);
    }
}
