use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct SectionPair{
    first: std::ops::RangeInclusive<u32>,
    second: std::ops::RangeInclusive<u32>,
}

impl SectionPair {
    fn fully_contained(&self) -> bool {
        if self.first.start() <= self.second.start() && self.first.end() >= self.second.end() {
            return true;
        }

        if self.second.start() <= self.first.start() && self.second.end() >= self.first.end() {
            return true;
        }

        return false;
    }

    fn overlaps(&self) -> bool {

        // This can be optimized iterating over the smaller RangeInclusive
        for value in self.first.clone() {
            if self.second.contains(&value){
                return true;
            }
        }

        return false;
    }
}

fn parse_file(file_path: &str) -> Vec<SectionPair> {
    let file = File::open(file_path).expect("Error opening given file");
    let lines = io::BufReader::new(file).lines();

    let mut pairs = vec![];

    for line in lines {
        let line = line.expect("Could not get current line as a string");

        let sections: Vec<&str> = line.split(",").collect();
        assert_eq!(sections.len(), 2, "Lines should have only two ranges splitted by a ','");
        let (first_part, second_part) = (sections[0], sections[1]);

        let first_part_splitted: Vec<&str> = first_part.split("-").collect();
        assert_eq!(first_part_splitted.len(), 2, "Each range has to be splitted by a -");
        let first_first: u32 = first_part_splitted[0].parse().unwrap();
        let first_second: u32 = first_part_splitted[1].parse().unwrap();

        let second_part_splitted: Vec<&str> = second_part.split("-").collect();
        assert_eq!(second_part_splitted.len(), 2, "Each range has to be splitted by a -");
        let second_first: u32 = second_part_splitted[0].parse().unwrap();
        let second_second: u32 = second_part_splitted[1].parse().unwrap();

        let curr_section = SectionPair {
            first: first_first ..= first_second,
            second: second_first ..= second_second,
        };

        pairs.push(curr_section);

    }

    return pairs;

}

fn how_many_pairs_fully_contained(file_path: &str) -> i32{
    let pairs = parse_file(file_path);
    return pairs.iter().filter(|pair| pair.fully_contained()).count() as i32;
}

fn how_many_pairs_overlap(file_path: &str) -> i32{
    let pairs = parse_file(file_path);
    return pairs.iter().filter(|pair| pair.overlaps()).count() as i32;
}


fn main() {
    let file_path = "./data/input.txt";
    let pairs_fully_contained = how_many_pairs_fully_contained(file_path);
    println!("Part one: {pairs_fully_contained}");

    let pairs_overlap = how_many_pairs_overlap(file_path);
    println!("Part two: {pairs_overlap}");
}

#[cfg(test)]
mod tests {
    use crate::{how_many_pairs_fully_contained, how_many_pairs_overlap};

    #[test]
    fn test_given_example_partone() {
        let file_path = "./data/input_example.txt";

        let computed = how_many_pairs_fully_contained(file_path);
        let expected = 2;
        assert_eq!(computed, expected, "The code is not correct in the given example");
    }

    #[test]
    fn test_given_example_parttwo() {
        let file_path = "./data/input_example.txt";

        let computed = how_many_pairs_overlap(file_path);
        let expected = 4;
        assert_eq!(computed, expected, "The code is not correct in the given example, for part two");
    }
}

