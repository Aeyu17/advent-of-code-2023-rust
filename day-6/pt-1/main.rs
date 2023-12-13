use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let times: Vec<u32> = contents[0].split(":")
                                     .map(String::from)
                                     .collect::<Vec<String>>()[1]
                                     .trim()
                                     .split_whitespace()
                                     .map(|x| x.parse::<u32>().unwrap())
                                     .collect();

    let distances: Vec<u32> = contents[1].split(":")
                                         .map(String::from)
                                         .collect::<Vec<String>>()[1]
                                         .trim()
                                         .split_whitespace()
                                         .map(|x| x.parse::<u32>().unwrap())
                                         .collect();

    let mut product = 1;

    for i in 0..times.len() {
        let mut successes = 0;

        let time = times[i];
        let distance = distances[i];

        for button_held in 0..=time {
            if button_held * (time - button_held) > distance {
                successes += 1;
            }
        }

        product *= successes;
    }

    println!("{}", product);
}