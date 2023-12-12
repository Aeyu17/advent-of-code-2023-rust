use std::{fs, convert::TryInto};
fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt").unwrap().lines().map(String::from).collect();

    let rows: usize = contents.len();

    let mut sum: usize = 0;

    for (row, &ref line) in contents.iter().enumerate() {
        let mut number_read = false;

        let cols: usize = line.chars().count();

        for (col, c) in line.chars().enumerate() {
            if !c.to_digit(10).is_some() {
                number_read = false;
                continue;
            } else if number_read {
                continue;
            } 

            let mut length: usize = 1;
            let mut number: usize = c.to_digit(10).unwrap().try_into().unwrap();
            number_read = true;

            while col+length < cols && String::from(line.as_bytes()[col+length] as char).parse::<usize>().is_ok() {
                number = number * 10 + String::from(line.as_bytes()[col+length] as char).parse::<usize>().unwrap();
                length += 1;
            }

            println!("{}, {}", number, length);

            if is_adjacent(&contents, row, col, length, &rows, &cols) {
                println!("Is adjacent");
                sum += number
            }
        }
    }

    println!("Sum: {}", sum);
}

fn is_adjacent(contents: &Vec<String>, row: usize, start_col: usize, num_len: usize, max_rows: &usize, max_cols: &usize) -> bool {
    if row == 0 && start_col == 0 {
        for i in row..=row+1 {
            for j in start_col..=start_col+num_len{
                let mut row_num: usize = i.try_into().unwrap();
                let mut col_num: usize = j.try_into().unwrap();
                if i >= *max_rows {
                    row_num = max_rows-1;
                }
    
                if j >= *max_cols {
                    col_num = max_cols-1;
                }
    
                let c: char = contents[row_num].as_bytes()[col_num] as char;
    
                if c != '.' && !c.to_digit(10).is_some() {
                    return true;
                }
            }
        }  
    } else if row == 0 {
        for i in row..=row+1 {
            for j in start_col-1..=start_col+num_len {
                let mut row_num: usize = i.try_into().unwrap();
                let mut col_num: usize = j.try_into().unwrap();
                if i >= *max_rows {
                    row_num = max_rows-1;
                }
    
                if j >= *max_cols {
                    col_num = max_cols-1;
                }
    
                let c: char = contents[row_num].as_bytes()[col_num] as char;
    
                if c != '.' && !c.to_digit(10).is_some() {
                    return true;
                }
            }
        }
    } else if start_col == 0 {
        for i in row-1..=row+1 {
            for j in start_col..=start_col+num_len {
                let mut row_num: usize = i.try_into().unwrap();
                let mut col_num: usize = j.try_into().unwrap();
                if i >= *max_rows {
                    row_num = max_rows-1;
                }
    
                if j >= *max_cols {
                    col_num = max_cols-1;
                }
    
                let c: char = contents[row_num].as_bytes()[col_num] as char;
    
                if c != '.' && !c.to_digit(10).is_some() {
                    return true;
                }
            }
        }
    } else {
        for i in row-1..=row+1 {
            for j in start_col-1..=start_col+num_len {
                let mut row_num: usize = i.try_into().unwrap();
                let mut col_num: usize = j.try_into().unwrap();
                if i >= *max_rows {
                    row_num = max_rows-1;
                }

                if j >= *max_cols {
                    col_num = max_cols-1;
                }

                let c: char = contents[row_num].as_bytes()[col_num] as char;

                if c != '.' && !c.to_digit(10).is_some() {
                    return true;
                }
            }
        }
    }
    return false;
}