use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let mut sum: usize = 0;

    for (row, &ref line) in contents.iter().enumerate() {

        let cols: usize = line.chars().count();

        for (col, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            } 

            let (a, b) = find_adjacent_nums(&contents, row, col, &cols);

            sum += a * b;
        }
    }

    println!("Sum: {}", sum);
}

fn find_adjacent_nums(contents: &Vec<String>, row: usize, col: usize, max_cols: &usize) -> (usize, usize) {
    let mut nums: (usize, usize) = (0, 0);
    if row == 0 && col == 0 {
        for i in row..=row+1 {
            for j in col..=col+1 {
                let c: char = contents[i].as_bytes()[j] as char;

                if !c.to_digit(10).is_some() {
                    continue;
                }

                println!("{}", c);

                let num = find_num(&contents[i], &j, &max_cols);

                if nums.0 == 0 {
                    nums.0 = num;
                } else if nums.0 != 0 && nums.0 != num {
                    nums.0 = num;
                }
            }
        }

    } else if row == 0 {
        for i in row..=row+1 {
            for j in col-1..=col+1 {
                let c: char = contents[i].as_bytes()[j] as char;

                if !c.to_digit(10).is_some() {
                    continue;
                }

                println!("{}", c);

                let num = find_num(&contents[i], &j, &max_cols);

                if nums.0 == 0 {
                    nums.0 = num;
                } else if nums.0 != 0 && nums.0 != num {
                    nums.0 = num;
                }
            }
        }

    } else if col == 0 {
        for i in row-1..=row+1 {
            for j in col..=col+1 {
                let c: char = contents[i].as_bytes()[j] as char;

                if !c.to_digit(10).is_some() {
                    continue;
                }

                println!("{}", c);

                let num = find_num(&contents[i], &j, &max_cols);

                if nums.0 == 0 {
                    nums.0 = num;
                } else if nums.0 != 0 && nums.0 != num {
                    nums.0 = num;
                }
            }
        }

    } else {
        for i in row-1..=row+1 {
            for j in col-1..=col+1 {
                let c: char = contents[i].as_bytes()[j] as char;

                if !c.to_digit(10).is_some() {
                    continue;
                }

                let num = find_num(&contents[i], &(j), &max_cols);

                if nums.0 == 0 || nums.0 == num {
                    nums.0 = num;
                } else if nums.1 == 0 || nums.1 == num {
                    nums.1 = num;
                } else {
                    return (0, 0);
                }
            }
        }
    }

    println!("{}, {}", nums.0, nums.1);

    return nums;
}

fn find_num(row_content: &String, col: &usize, max_cols: &usize) -> usize {
    let num_char: char = row_content.as_bytes()[*col] as char;
    let mut num_str: String = num_char.to_string();

    for c in row_content[col+1..*max_cols].to_string().chars() {
        if c.to_digit(10).is_some() {
            num_str = num_str + &c.to_string();
        } else {
            break;
        }
    }

    for c in row_content[0..=col-1].to_string().chars().rev() {
        if c.to_digit(10).is_some() {
            num_str = c.to_string() + &num_str;
        } else {
            break;
        }
    }

    let num = num_str.parse::<usize>().unwrap();

    println!("Found number: {}", num);

    return num;
}