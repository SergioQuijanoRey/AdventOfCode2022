use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
pub struct ElveBag {
    items: Vec<i32>,
}

impl ElveBag{
    pub fn sum(&self) -> i32{
        return self.items.iter().sum();
    }
}

pub fn parse_file(file_path: &str) -> Vec<ElveBag> {
    let file = File::open(file_path).expect("Error opening given file");
    let lines = io::BufReader::new(file).lines();

    let mut current_bag: Vec<i32> = vec![];
    let mut elve_bags: Vec<ElveBag> = vec![];

    for line in lines{
        let line = line.expect("Error reading current line of the file");

        // We found a newline, so create a new elve bag and append it to
        // our list of elve bags
        if line == "" {
            let new_bag = ElveBag{items: current_bag.clone()};
            elve_bags.push(new_bag);
            current_bag = vec![];
            continue;
        }

        // No new line, so get the integer and store it in the current bag
        let current_food: i32 = line.parse().expect("Could not get integer from current line");
        current_bag.push(current_food);
    }

    // Check for the last bag
    if current_bag.len() > 0 {
        let new_bag = ElveBag{items: current_bag.clone()};
        elve_bags.push(new_bag);
    }

    return elve_bags;
}

pub fn get_position_of_elve_with_more_food(bags: &Vec<ElveBag>) -> usize {
    let food_sums: Vec<i32> = bags.iter().map(|bag| bag.sum()).collect();
    let max_value = food_sums.iter().max().expect("Could not get the max value of the food sums");
    let max_position = food_sums.iter().position(|val| val == max_value).expect("Could not get position of max value");

    // Counting starting by one
    return max_position + 1;
}

pub fn get_max_food(bags: &Vec<ElveBag>) -> i32{
    let food_sums: Vec<i32> = bags.iter().map(|bag| bag.sum()).collect();
    let max_value = food_sums.iter().max().expect("Could not get the max value of the food sums");

    return *max_value;
}

pub fn sum_calories_top_three(bags: &Vec<ElveBag>) -> i32 {
    // Sum the calories in the bags
    let mut food_sums: Vec<i32> = bags.iter().map(|bag| bag.sum()).collect();

    // Inverse sort by the sum of the calories
    food_sums.sort_by(|a, b| b.cmp(a));

    // Return the sum of the greates three bags
    let food_sums_sorted_top_three = &food_sums[0..3];
    return food_sums_sorted_top_three.iter().sum();
}

fn main() {
    // Parse the input
    let input_data_file = "./data/input.txt";
    let elve_bags = parse_file(input_data_file);

    // Get the position of elve with highest food and the value
    let position = get_position_of_elve_with_more_food(&elve_bags);
    let value = get_max_food(&elve_bags);
    println!("Answer I: pos {position} with food {value}");

    // Compute the sum of the top three calories
    let sum_top_three = sum_calories_top_three(&elve_bags);
    println!("Answer II: {sum_top_three}");

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_given_example(){
        // Parse the input
        let input_data_file = "./data/test_input.txt";
        let elve_bags = parse_file(input_data_file);

        // Get the position of elve with highest food
        let position = get_position_of_elve_with_more_food(&elve_bags);

        assert_eq!(position, 4, "The code does not work with the given example!");

    }

    #[test]
    fn test_given_example_part_two(){
        // Parse the input
        let input_data_file = "./data/test_input.txt";
        let elve_bags = parse_file(input_data_file);

        let expected = 45000;
        let computed = sum_calories_top_three(&elve_bags);
        assert_eq!(expected, computed, "Part two is not correct!");
    }
}
