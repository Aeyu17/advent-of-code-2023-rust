use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let time: u64 = contents[0].split(":")
                                     .map(String::from)
                                     .collect::<Vec<String>>()[1]
                                     .trim()
                                     .split_whitespace()
                                     .map(String::from)
                                     .collect::<Vec<String>>()
                                     .iter()
                                     .fold("".to_string(), |acc, x| acc.to_owned() + x)
                                     .parse::<u64>()
                                     .unwrap();

    let distance: u64 = contents[1].split(":")
                                         .map(String::from)
                                         .collect::<Vec<String>>()[1]
                                         .trim()
                                         .split_whitespace()
                                         .map(String::from)
                                         .collect::<Vec<String>>()
                                         .iter()
                                         .fold("".to_string(), |acc, x| acc.to_owned() + x)
                                         .parse::<u64>()
                                         .unwrap();

    let mut successes = 0;



    for button_held in 0..=time {
        if button_held * (time - button_held) > distance {
            successes += 1;
        }
    }

    println!("{}", successes);


}