use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let mut sum: u32 = 0;

    for line in contents {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for (_, hand) in line.split(':').map(String::from).collect::<Vec<String>>()[1].split(';').map(String::from).collect::<Vec<String>>().iter().enumerate() {
            

            for (_, group) in hand.split(',').map(String::from).collect::<Vec<String>>().iter().enumerate(){
                let group_vec = group.trim().split_whitespace().map(String::from).collect::<Vec<String>>();
                let count = group_vec[0].parse::<u32>().unwrap();
                let color = &group_vec[1];

                if color == "red" && count > min_red {
                    min_red = count;
                }

                if color == "green" && count > min_green {
                    min_green = count;
                }

                if color == "blue" && count > min_blue {
                    min_blue = count;
                }

                println!("{}, {}", count, color)
            }
        }
        sum += min_red * min_green * min_blue
    }

    println!("Game sums: {}", sum);
}