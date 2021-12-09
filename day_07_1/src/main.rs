fn needed_fuel(crabs: &[usize], position: usize) -> usize {
    crabs.iter().fold(0, |acc, &c| {
        (position as isize - c as isize).abs() as usize + acc
    })
}

fn best_align_position(crabs: &[usize]) -> usize {
    median(crabs)
}

fn main() {
    let crabs = parse_crabs(include_str!("../input.txt"));
    println!("{:?}", crabs);
    println!("{}", needed_fuel(&crabs, best_align_position(&crabs)));
}

fn parse_crabs(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn median(numbers: &[usize]) -> usize {
    let mut numbers = numbers.to_vec();
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let crabs = parse_crabs(include_str!("../input_test.txt"));
        println!("{:?}", crabs);
        assert_eq!(needed_fuel(&crabs, best_align_position(&crabs)), 37);
    }

    #[test]
    fn test_best_align_position() {
        let crabs = parse_crabs(include_str!("../input_test.txt"));
        assert_eq!(best_align_position(&crabs), 2);
    }
}
