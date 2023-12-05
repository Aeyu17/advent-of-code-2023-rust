use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let mut sum: u32 = 0;

    for line in contents {
        let game_num: u32 = line[5..line.find(":").unwrap()].parse().unwrap();
        let mut game_invalid: bool = false;

        for (_, hand) in line.split(':').map(String::from).collect::<Vec<String>>()[1].split(';').map(String::from).collect::<Vec<String>>().iter().enumerate() {

            for (_, group) in hand.split(',').map(String::from).collect::<Vec<String>>().iter().enumerate(){
                let group_vec = group.trim().split_whitespace().map(String::from).collect::<Vec<String>>();
                let count = group_vec[0].parse::<u32>().unwrap();
                let color = &group_vec[1];

                if (color == "red" && count > 12) || (color == "green" && count > 13) || (color == "blue" && count > 14) {
                    game_invalid = true;
                    break;
                } 

                println!("{}, {}", count, color)
            }

            if game_invalid {
                break;
            }
        }

        if !game_invalid {
            sum += game_num;
        }
    }

    println!("Game sums: {}", sum);
}