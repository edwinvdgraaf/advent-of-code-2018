use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn run() -> String {
    let input = read_file().unwrap();
    format!("part one: {:?} | part two: {:?}", compute_part_one(&input), compute_part_two(&input))
}

fn read_file() ->  std::io::Result<(String)> {   
    let mut file = File::open("src/day_two/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn compute_part_one(input: &String) -> i32 {
    let mut duos = 0;
    let mut trios = 0;

    for line in input.lines() {
        let line_stats = count_chars_in_line(line);
        let mut found_duo_for_line = false;
        let mut found_trio_for_line = false;

        for (&_character, &amount) in &line_stats {
           if amount == 2 && !found_duo_for_line {
               duos += 1;
               found_duo_for_line = true;
           }

           if amount == 3 && !found_trio_for_line {
               trios += 1;
               found_trio_for_line = true;
           }
        }
    }
    duos * trios
}

fn compute_part_two(input: &String) -> String {
    let mut output: String = "".to_string();
    
    for line in input.lines() {
        for compare_line in input.lines() {
            let distance = get_string_difference(line, compare_line);

            if distance == 1 {
                output = get_chars_in_both(line, compare_line).unwrap();
            }
        }
    }
    output.to_string()
}

fn count_chars_in_line(line: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();

    for character in line.to_owned().to_string().chars() { 
        let counter = char_count.entry(character).or_insert(0);
        *counter += 1;
    }

    char_count
}

fn get_string_difference(a: &str, b: &str) -> i32 {
    if a.eq(b) {
        return 0;
    }
    
    if a.len() != b.len() {
        return -1;
    }

    a.chars()
        .zip(b.chars())
        .map(|(a_char, b_char)| if a_char.eq(&b_char) { 0 } else { 1 })
        .sum()
}

fn get_chars_in_both(a: &str, b: &str) -> Option<String> {
    if a.eq(b) {
        return Some(a.to_owned());
    }
    
    if a.len() != b.len() {
        return None;
    }

    let mut same_chars = vec![];

    a.chars()
        .zip(b.chars())
        .for_each(|(a_char, b_char)| {
            if a_char.eq(&b_char) { 
                same_chars.push(a_char)
            };
        });

    Some(same_chars.into_iter().collect())
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
    fn it_computes_diff_ids() {
        assert_eq!(0, get_string_difference("abc", "abc"));
        assert_eq!(-1, get_string_difference("abcd", "abc"));
        assert_eq!(1, get_string_difference("abc", "acc"));
        assert_eq!(2, get_string_difference("abcf", "abde"));
    }

    #[test]
    fn it_computes_part_two() {
        let output = compute_part_two(&"abcde\r\nfghij\r\nklmno\r\npqrst\r\nfguij\r\naxcye\r\nwvxyz\r\n".to_string());
        assert_eq!("fgij", output);
    }
}
