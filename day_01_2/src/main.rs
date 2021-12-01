use itertools::Itertools;
use std::fs;

fn get_increases(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .tuple_windows::<(_, _, _)>()
        .tuple_windows::<(_, _)>()
        .filter(|((a, b, c), (d, e, f))| a + b + c < d + e + f)
        .count()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("{}", get_increases(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_increases() {
        let input_test = "
199
200
208
210
200
207
240
269
260
263
";
        assert_eq!(get_increases(input_test), 5);
    }
}
