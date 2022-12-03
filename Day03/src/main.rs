use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Rucksack {
    first: String,
    second: String,
}

fn map_to_int(c: char) -> i32 {
    if c >= 'a' && c <= 'z'{
        let diff = c as u32 - 'a' as u32;
        return diff as i32 + 1;
    }

    let diff = c as u32 - 'A' as u32;
    return diff as i32 + 27;
}

impl Rucksack {
    fn priority(&self) -> i32 {
        let repeated_char = self.get_repeated_char();
        return map_to_int(repeated_char);
    }

    fn get_repeated_char(&self) -> char {
        if self.first.len() != self.second.len(){
            panic!("Two compartiments must be the same size!");
        }

        for i in 0..self.first.len(){
            let first_char = self.first.chars().nth(i).unwrap();
            if self.second.contains(first_char) {
                return first_char;
            }
        }

        // We didn't find a repeated char
        panic!("We did not find a repeated char!");
    }

    fn get_string(&self) -> String {
        return format!("{}{}", self.first, self.second);
    }
}

#[derive(Debug)]
struct Group{
    first: Rucksack,
    second: Rucksack,
    third: Rucksack,
}
impl Group {
    fn priority(&self) -> i32 {
        let repeated_char = self.get_repeated_char();
        return map_to_int(repeated_char);
    }

    fn get_repeated_char(&self) -> char {

        let first_str = self.first.get_string();
        let second_str = self.second.get_string();
        let third_str = self.third.get_string();

        for first_char in first_str.chars(){
            if second_str.contains(first_char) && third_str.contains(first_char) {
                return first_char;
            }
        }

        // We didn't find a repeated char
        panic!("We did not find a repeated char!");

    }
}

fn parse_file(file_path: &str) -> Vec<Rucksack> {
    let file = File::open(file_path).expect("Error opening given file");
    let lines = io::BufReader::new(file).lines();

    let mut rucksacks = vec![];

    for line in lines{
        let line = line.expect("Error reading current line of the file");

        let (first, second) = line.split_at(line.len() / 2);
        assert_eq!(first.len(), second.len(), "The two rucksacks compartiments must have the same len");

        let rucksack = Rucksack{first: first.to_string(), second: second.to_string()};
        rucksacks.push(rucksack);
    }

    return rucksacks;
}


fn compute_sum_priorities(file_path: &str) -> i32{
    let rucksacks = parse_file(file_path);
    return rucksacks.iter().map(|ruck| ruck.priority()).sum();
}

fn compute_sum_badges(file_path: &str) -> i32 {
    let rucksacks = parse_file(file_path);
    let groups = group_rucksacks(rucksacks);
    return groups.iter().map(|group| group.priority()).sum();

}

fn group_rucksacks(rucksacks: Vec<Rucksack>) -> Vec<Group> {
    let mut groups = vec![];
    let mut curr_group = vec![];

    for ruck in rucksacks {
        curr_group.push(ruck);

        if curr_group.len() == 3{
            groups.push(Group {
                first: curr_group[0].clone(),
                second: curr_group[1].clone(),
                third: curr_group[2].clone(),
            });

            curr_group = vec![];
        }
    }

    return groups;
}

fn main() {
    let file_path = "data/input.txt";
    let priority = compute_sum_priorities(file_path);
    println!("Priority (part one): {priority}");

    let sum_badges = compute_sum_badges(file_path);
    println!("Sum badges (part two): {sum_badges}");
}

#[cfg(test)]
mod tests {
    use crate::{compute_sum_priorities, map_to_int, compute_sum_badges};

    #[test]
    fn test_given_example_partone() {
        let file_path = "data/input_example.txt";

        let expected = 157;
        let computed = compute_sum_priorities(file_path);

        assert_eq!(expected, computed, "Code is not computing well the given example!");
    }

    #[test]
    fn test_map_to_int() {
        let expected = 1;
        let computed = map_to_int('a');
        assert_eq!(expected, computed, "Map to int failed in a basic case!");

        let expected = 3;
        let computed = map_to_int('c');
        assert_eq!(expected, computed, "Map to int failed in a basic case!");

        let expected = 27;
        let computed = map_to_int('A');
        assert_eq!(expected, computed, "Map to int failed in a basic case!");

        let expected = 30;
        let computed = map_to_int('D');
        assert_eq!(expected, computed, "Map to int failed in a basic case!");
    }

    #[test]
    fn test_given_example_parttwo() {
        let file_path = "data/input_example.txt";

        let expected = 70;
        let computed = compute_sum_badges(file_path);

        assert_eq!(expected, computed, "Code is not computing well the given example!");
    }

}
