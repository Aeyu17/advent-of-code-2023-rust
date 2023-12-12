use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let mut card_count: Vec<u32> = vec![0; contents.len()];

    for _ in 1..0 {
        println!("test");
    }

    for (i, card) in contents.iter().enumerate() {
        card_count[i] += 1;
        let mut card_val = 0;
        let winning_nums: Vec<u32> = card.split(":").map(String::from).collect::<Vec<String>>()[1].split("|").map(String::from).collect::<Vec<String>>()[0].trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let obtained_nums: Vec<u32> = card.split(":").map(String::from).collect::<Vec<String>>()[1].split("|").map(String::from).collect::<Vec<String>>()[1].trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        for num in obtained_nums.iter() {
            if winning_nums.contains(&num) {
                card_val += 1;
            }
        }

        for _ in 0..card_count[i] {
            for j in 1..card_val+1 {
                card_count[i+j] += 1;
            }
        }
    }

    let sum: u32 = card_count.iter().sum();

    println!("Sum: {}", sum);
}