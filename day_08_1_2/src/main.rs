use std::collections::HashMap;

use std::str::FromStr;

struct Entry {
    digits: Vec<String>,
    numbers: Vec<u8>,
}

fn contains_same_letter(longest: &str, shortest: &str) -> bool {
    shortest.chars().all(|c| longest.contains(c))
}

fn find_one(digits: &mut Vec<String>) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .find(|(_idx, d)| d.len() == 2)
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_four(digits: &mut Vec<String>) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .find(|(_idx, d)| d.len() == 4)
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_seven(digits: &mut Vec<String>) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .find(|(_idx, d)| d.len() == 3)
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_eight(digits: &mut Vec<String>) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .find(|(_idx, d)| d.len() == 7)
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_three(digits: &mut Vec<String>, one: String) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .filter(|(_idx, d)| d.len() == 5)
        .find(|(_idx, d)| contains_same_letter(*d, &one))
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_nine(digits: &mut Vec<String>, three: String) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .filter(|(_idx, d)| d.len() == 6)
        .find(|(_idx, d)| contains_same_letter(*d, &three))
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_five(digits: &mut Vec<String>, nine: String) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .filter(|(_idx, d)| d.len() == 5)
        .find(|(_idx, d)| contains_same_letter(&nine, *d))
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_two(digits: &mut Vec<String>) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .filter(|(_idx, d)| d.len() == 5)
        .nth(0)
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_six(digits: &mut Vec<String>, five: String) -> String {
    let local_digits = digits.clone();
    let (idx, digit) = local_digits
        .iter()
        .enumerate()
        .filter(|(_idx, d)| d.len() == 6)
        .find(|(_idx, d)| contains_same_letter(*d, &five))
        .unwrap();
    digits.remove(idx);
    digit.to_string()
}

fn find_zero(digits: &mut Vec<String>) -> String {
    digits.iter().nth(0).unwrap().to_string()
}

fn extract_numbers(digits: Vec<String>) -> HashMap<String, u8> {
    let mut digits = digits
        .iter()
        .map(|d| {
            let mut sorted = d.clone().chars().collect::<Vec<char>>();
            sorted.sort();
            sorted.iter().collect::<String>()
        })
        .collect::<Vec<String>>();
    let mut digit_map = HashMap::new();
    let one = find_one(&mut digits);
    digit_map.insert(one.clone(), 1);
    digit_map.insert(find_four(&mut digits), 4);
    digit_map.insert(find_seven(&mut digits), 7);
    digit_map.insert(find_eight(&mut digits), 8);
    let three = find_three(&mut digits, one);
    digit_map.insert(three.clone(), 3);
    let nine = find_nine(&mut digits, three);
    digit_map.insert(nine.clone(), 9);
    let five = find_five(&mut digits, nine);
    digit_map.insert(five.clone(), 5);
    digit_map.insert(find_two(&mut digits), 2);
    digit_map.insert(find_six(&mut digits, five), 6);
    digit_map.insert(find_zero(&mut digits), 0);
    digit_map
}

impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ");
        let signal_patterns: Vec<String> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let digit_map = extract_numbers(signal_patterns);

        let digits: Vec<String> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let numbers = digits
            .iter()
            .map(|s| {
                let mut sorted = s.clone().chars().collect::<Vec<char>>();
                sorted.sort();
                let sorted = sorted.iter().collect::<String>();
                digit_map.get(&sorted).unwrap().clone()
            })
            .collect();

        Ok(Self { digits, numbers })
    }
}

impl Entry {
    fn get_value(&self) -> u32 {
        self.numbers.iter().fold(0, |acc, n| acc * 10 + *n as u32)
    }
}

fn solve(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|e| {
            e.digits
                .iter()
                .map(|d| d.len())
                .filter(|&l| l == 2 || l == 4 || l == 3 || l == 7)
                .count()
        })
        .sum()
}

fn parse_entries(input: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in input.lines() {
        entries.push(line.parse().unwrap());
    }
    entries
}

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_entries(input);
    println!("Digits: {}", solve(&input));
    println!(
        "Solved sum: {}",
        input.iter().map(|e| e.get_value()).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input_test.txt");
        let input = parse_entries(input);
        assert_eq!(solve(&input), 26);
        assert_eq!(input.get(0).unwrap().get_value(), 8394);
        assert_eq!(input.get(1).unwrap().get_value(), 9781);
        assert_eq!(input.get(2).unwrap().get_value(), 1197);
        assert_eq!(input.get(3).unwrap().get_value(), 9361);
        assert_eq!(input.get(4).unwrap().get_value(), 4873);
        assert_eq!(input.get(5).unwrap().get_value(), 8418);
        assert_eq!(input.get(6).unwrap().get_value(), 4548);
        assert_eq!(input.get(7).unwrap().get_value(), 1625);
        assert_eq!(input.get(8).unwrap().get_value(), 8717);
        assert_eq!(input.get(9).unwrap().get_value(), 4315);
    }
}
