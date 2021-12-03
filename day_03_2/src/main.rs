use std::fs;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Rating {
    Oxygen,
    Co2,
}

fn bit_criteria(input: &str, rating: Rating, rank: usize) -> char {
    let lines: Vec<&str> = input.lines().collect();
    let count_zeros = lines
        .iter()
        .map(|line| line.chars().nth(rank).unwrap())
        .filter(|c| *c == '1')
        .count();
    if count_zeros as f32 >= lines.len() as f32 / 2.0 {
        match rating {
            Rating::Oxygen => '0',
            Rating::Co2 => '1',
        }
    } else {
        match rating {
            Rating::Oxygen => '1',
            Rating::Co2 => '0',
        }
    }
}

fn filter(input: &str, c: char, rank: usize) -> String {
    input
        .lines()
        .filter(|line| line.chars().nth(rank).unwrap() == c)
        .collect::<Vec<&str>>()
        .join("\n")
}

fn get_rating(input: &str, rating: Rating, rank: usize) -> usize {
    let c = bit_criteria(input, rating, rank);
    let filtered = filter(input, c, rank);
    if filtered.lines().count() == 1 {
        usize::from_str_radix(&filtered, 2).expect("Invalid input")
    } else {
        get_rating(&filtered, rating, rank + 1)
    }
}

fn oxygen_rating(input: &str) -> usize {
    get_rating(input, Rating::Oxygen, 0)
}

fn co2_rating(input: &str) -> usize {
    get_rating(input, Rating::Co2, 0)
}

fn life_support_rating(input: &str) -> usize {
    let o2 = oxygen_rating(input);
    let co2 = co2_rating(input);
    o2 * co2
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{}", life_support_rating(&contents));
}

#[cfg(test)]
mod tests {
    use crate::life_support_rating;

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
        assert_eq!(life_support_rating(input), 230);
    }
}
