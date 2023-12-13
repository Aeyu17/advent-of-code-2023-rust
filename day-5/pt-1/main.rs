use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt")
                                    .unwrap()
                                    .lines()
                                    .map(String::from)
                                    .filter(|x| x.len() > 0)
                                    .collect();

    let seeds: Vec<u64> = contents[0].split(": ")
                                     .map(String::from)
                                     .collect::<Vec<String>>()[1]
                                     .split_whitespace()
                                     .map(|x| x.parse::<u64>().unwrap())
                                     .collect();

    let mut min: Option<u64> = None;

    let transformation_lists = parse_data(contents);

    for seed in seeds.iter() {
        let mut location = *seed;
        for transformation_set in transformation_lists.iter() {
            location = transformation_set.transform(location);
        }

        if !min.is_some() || min.unwrap() > location {
            min = Some(location);
        }
    }

    println!("Minimum location: {}", min.unwrap());

}

fn parse_data(contents: Vec<String>) -> Vec<TransformationSet> {
    let mut transform_list = vec![TransformationSet::new(); 7];
    let mut index = 0;
    let mut flag = false;

    for line in contents.iter() {
        if line.contains("seeds:") {
            continue;
        }

        if line.contains("map") {
            // ignore first runthrough
            if flag {
                index += 1;
            } else {
                flag = true;
            }
            
            continue;
        }

        transform_list[index].push(Transformation::to_transform(line));
    }

    return transform_list;
}

#[derive(Clone)]
struct Transformation {
    source_start: u64,
    dest_start: u64,
    length: u64
}

impl Transformation {
    fn new(source_start: u64, dest_start: u64, length: u64) -> Transformation {
        Transformation{source_start: source_start, dest_start: dest_start, length: length}
    }

    pub fn to_transform(line: &String) -> Transformation {
        let data = line.split_whitespace()
                       .map(|x| x.parse::<u64>().unwrap())
                       .collect::<Vec<u64>>();

        return Transformation::new(data[1], data[0], data[2]);
    }

    fn is_within(&self, num: u64) -> bool {
        return num >= self.source_start && num < self.source_start + self.length;
    }

    pub fn transform(&self, num: u64) -> u64 {
        if Transformation::is_within(self, num) {
            return num + self.dest_start - self.source_start;
        }

        return num;
    }
}

#[derive(Clone)]
struct TransformationSet {
    transforms: Vec<Transformation>
}

impl TransformationSet {
    pub fn new() -> TransformationSet{
        TransformationSet{transforms: Vec::new()}
    }

    pub fn push(&mut self, t: Transformation) {
        self.transforms.push(t);
    }

    pub fn transform(&self, num: u64) -> u64 {
        for transform in self.transforms.iter() {

            if transform.is_within(num) {

                return transform.transform(num);
            }
        }
        return num;
    }
}