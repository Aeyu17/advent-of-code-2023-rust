use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Invalid path name.");

    let mut sum: u32 = 0;

    for line in contents.split_whitespace() {
        let mut tens: u32 = 0;
        let mut ones: u32 = 0;

        for c in line.chars() {
            if c.to_digit(10).is_some() {
                tens = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.to_digit(10).is_some() {
                ones = c.to_digit(10).unwrap();
                break;
            }
        }

        sum += tens * 10 + ones;
    }

    println!("{}", sum);
}