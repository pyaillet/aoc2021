use std::fs;

fn transpose(input: &str) -> (Vec<Vec<char>>, usize) {
    let lines = input.lines().collect::<Vec<_>>();
    let length = lines.iter().map(|line| line.len()).min().unwrap();
    (
        (0..length)
            .map(|idx| {
                lines
                    .iter()
                    .filter_map(|line| line.chars().nth(idx))
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
        lines.len(),
    )
}

fn get_power_consumption(input: &str) -> usize {
    let (tr, size) = transpose(input);
    let gamma_rate = tr
        .iter()
        .map(|line| {
            if line.iter().filter(|c| **c == '1').count() > (size / 2) {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    let epsilon = gamma_rate
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => '0',
        })
        .collect::<String>();
    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).expect("invalid gamma rate");
    let epsilon = usize::from_str_radix(&epsilon, 2).expect("invalid epsilon");
    gamma_rate * epsilon
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{}", get_power_consumption(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(get_power_consumption(input), 198);
    }
}
