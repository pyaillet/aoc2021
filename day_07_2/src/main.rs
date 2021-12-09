fn needed_fuel_for_pos(crabs: &[usize], position: usize) -> usize {
    crabs.iter().fold(0, |acc, &c| {
        let nb_move = (position as isize - c as isize).abs() as usize;
        (nb_move * (nb_move + 1) / 2) + acc
    })
}

fn needed_fuel(crabs: &[usize]) -> usize {
    (*crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap())
        .map(|p| needed_fuel_for_pos(&crabs, p))
        .min()
        .unwrap()
}

fn best_align_position(crabs: &[usize]) -> usize {
    average(crabs)
}

fn main() {
    let crabs = parse_crabs(include_str!("../input.txt"));
    println!("{:?}", crabs);
    println!("{}", needed_fuel(&crabs));
}

fn parse_crabs(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn average(numbers: &[usize]) -> usize {
    (numbers.iter().sum::<usize>() as f32 / numbers.len() as f32).round() as usize
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
        assert_eq!(needed_fuel(&crabs), 168);
    }

    #[test]
    fn test_best_align_position() {
        let crabs = parse_crabs(include_str!("../input_test.txt"));
        assert_eq!(best_align_position(&crabs), 5);
    }
}
