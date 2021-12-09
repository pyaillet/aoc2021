use std::str::FromStr;

#[derive(Clone, Debug)]
struct Fishes(Vec<u8>);

impl Iterator for Fishes {
    type Item = Fishes;
    fn next(&mut self) -> Option<Self::Item> {
        let mut new_vec = self.0.clone();
        for i in 0..self.0.len() {
            if self.0[i] == 0 {
                new_vec[i] = 6;
                new_vec.push(8);
            } else {
                new_vec[i] -= 1;
            }
        }

        self.0 = new_vec.clone();

        Some(new_vec.into())
    }
}

impl From<Vec<u8>> for Fishes {
    fn from(v: Vec<u8>) -> Self {
        Fishes(v)
    }
}

impl FromStr for Fishes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(",")
            .filter_map(|f| f.parse::<u8>().ok())
            .collect::<Vec<u8>>()
            .into())
    }
}

fn count_lantern_fishes(input: &str, days: usize) -> usize {
    let mut fishes = Fishes::from_str(input).unwrap();
    fishes.nth(days - 1).unwrap().0.len()
}

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Lantern fishes: {}", count_lantern_fishes(input, 80));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input_test.txt").trim();
        assert_eq!(count_lantern_fishes(input, 18), 26);
        assert_eq!(count_lantern_fishes(input, 80), 5934);
    }
}
