use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Invalid path name.");

    let mut sum: i32 = 0;
    let num_strings = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in contents.split_whitespace() {
        let mut tens: i32 = -1;
        let mut ones: i32 = -1;

        for (i, &item) in line.as_bytes().iter().enumerate() {
            let c = item as char;
            if c.to_digit(10).is_some() {
                if tens == -1 {
                    tens = c.to_digit(10).unwrap() as i32;
                }
                ones = c.to_digit(10).unwrap() as i32;

            } else if i+3 <= line.len() && num_strings.contains_key(&line[i..i+3]) {
                let h = &line[i..i+3];
                if tens == -1 {
                    tens = *num_strings.get(&line[i..i+3]).unwrap();
                }
                ones = *num_strings.get(&line[i..i+3]).unwrap();

            } else if i+4 <= line.len() && num_strings.contains_key(&line[i..i+4]) {
                if tens == -1 {
                    tens = *num_strings.get(&line[i..i+4]).unwrap();
                }
                ones = *num_strings.get(&line[i..i+4]).unwrap();

            } else if i+5 <= line.len() && num_strings.contains_key(&line[i..i+5]) {
                if tens == -1 {
                    tens = *num_strings.get(&line[i..i+5]).unwrap();
                }
                ones = *num_strings.get(&line[i..i+5]).unwrap();
            }
        }

        sum += tens * 10 + ones;
    }

    println!("{}", sum);
}