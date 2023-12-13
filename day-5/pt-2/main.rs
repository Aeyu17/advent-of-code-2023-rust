use std::fs;

fn main() {
    let contents: Vec<String> = fs::read_to_string("../input.txt")
                                    .unwrap()
                                    .lines()
                                    .map(String::from)
                                    .filter(|x| x.len() > 0)
                                    .collect();

    let seed_list: Vec<u64> = contents[0].split(": ")
                                     .map(String::from)
                                     .collect::<Vec<String>>()[1]
                                     .split_whitespace()
                                     .map(|x| x.parse::<u64>().unwrap())
                                     .collect();

    let mut min: Option<u64> = None;

    let transformation_lists = parse_data(contents);

    for (i, num) in seed_list.iter().enumerate() {
        if i % 2 == 1 {
            continue;
        }

        let source_start = *num;
        let range = seed_list[i+1];

        let mut seed_pile = Vec::from([source_start, source_start + range]);

        for transformation_set in transformation_lists.iter() {
            let mut transformed_pile = vec![0; seed_pile.len()];

            println!("A: {:?}", seed_pile);

            for (i, seed) in seed_pile.iter().enumerate() {
                transformed_pile[i] = transformation_set.transform(seed);
            }

            println!("B: {:?}", transformed_pile);

            let mut new_seeds = Vec::new();

            for i in 0..seed_pile.len()-1 {
                new_seeds.push(transformed_pile[i]);

                if i % 2 == 1 {
                    continue;
                }

                if transformed_pile[i] > transformed_pile[i+1] && seed_pile[i] > seed_pile[i+1] && transformed_pile[i] - transformed_pile[i+1] != seed_pile[i] - seed_pile[i+1] // negative distance doesn't match
                || transformed_pile[i+1] > transformed_pile[i] && seed_pile[i+1] > seed_pile[i] && transformed_pile[i+1] - transformed_pile[i] != seed_pile[i+1] - seed_pile[i] // positive distance doesn't match
                || transformed_pile[i+1] > transformed_pile[i] && seed_pile[i+1] < seed_pile[i] || transformed_pile[i+1] < transformed_pile[i] && seed_pile[i+1] > seed_pile[i]{ // signs don't match
                    new_seeds.extend(transformation_set.find_boundaries(seed_pile[i], seed_pile[i+1])
                                                       .iter()
                                                       .map(|x| transformation_set.transform(x))
                                                       .collect::<Vec<u64>>());
                }

            }

            println!("");

            new_seeds.push(transformed_pile[transformed_pile.len()-1]);

            seed_pile = new_seeds;            
        }

        println!("Final: {:?}", seed_pile);

        if !min.is_some() || min.unwrap() > *seed_pile.iter().min().unwrap() {
            min = Some(*seed_pile.iter().min().unwrap());
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

    fn is_within(&self, num: &u64) -> bool {
        return *num >= self.source_start && *num < self.source_start + self.length;
    }

    pub fn transform(&self, num: &u64) -> u64 {
        if self.is_within(num) {
            return *num + self.dest_start - self.source_start;
        }

        return *num;
    }
}

#[derive(Clone)]
struct TransformationSet {
    transforms: Vec<Transformation>,
    boundaries: Vec<u64>
}

impl TransformationSet {
    pub fn new() -> TransformationSet{
        TransformationSet{transforms: Vec::new(), boundaries: Vec::new()}
    }

    pub fn push(&mut self, t: Transformation) {
        self.transforms.push(t.clone());
        self.boundaries.push(t.source_start);
        self.boundaries.push(t.source_start + t.length);
        self.boundaries.sort();
        self.boundaries.dedup();
    }

    pub fn transform(&self, num: &u64) -> u64 {
        for transform in self.transforms.iter() {

            if transform.is_within(num) {

                return transform.transform(num);
            }
        }
        
        return *num;
    }

    pub fn find_boundaries(&self, start: u64, end: u64) -> Vec<u64> {
        println!("Bounds: {}, {}", start, end);
        let mut boundary_list: Vec<u64> = Vec::new();

        for boundary in self.boundaries.iter() {
            if *boundary >= start && *boundary <= end || *boundary <= start && *boundary >= end {
                if *boundary != 0 {
                    boundary_list.push(*boundary-1);
                } else {
                    boundary_list.push(0);
                }
                
                boundary_list.push(*boundary);
            } 
        }  

        println!("Adding: {:?}", boundary_list);
        return boundary_list;
    }
}