use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let mut sum = 0;

    for card in contents.iter() {
        let mut card_val = 0;
        let winning_nums: Vec<u32> = card.split(":").map(String::from).collect::<Vec<String>>()[1].split("|").map(String::from).collect::<Vec<String>>()[0].trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let obtained_nums: Vec<u32> = card.split(":").map(String::from).collect::<Vec<String>>()[1].split("|").map(String::from).collect::<Vec<String>>()[1].trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        for num in obtained_nums.iter() {
            if winning_nums.contains(&num) {
                if card_val == 0 {
                    card_val = 1;
                } else {
                    card_val *= 2;
                }
            }
        }

        sum += card_val;
    }

    println!("Sum: {}", sum);
}