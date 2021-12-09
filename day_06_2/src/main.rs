use std::str::FromStr;

#[derive(Clone, Debug)]
struct Fishes([usize; 9]);

impl Iterator for Fishes {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        let mut new: [usize; 9] = [0; 9];
        for i in 0..9 {
            if i == 0 {
                new[8] = self.0[i];
                new[6] = self.0[i];
            } else {
                new[i - 1] += self.0[i];
            }
        }
        self.0 = new;
        Some(())
    }
}

impl From<Vec<u8>> for Fishes {
    fn from(v: Vec<u8>) -> Self {
        v.iter().fold(Fishes([0; 9]), |mut acc, x| {
            acc.0[*x as usize] += 1;
            acc
        })
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
    fishes.nth(days - 1).unwrap();
    fishes.0.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Lantern fishes: {}", count_lantern_fishes(input, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input_test.txt").trim();
        assert_eq!(count_lantern_fishes(input, 18), 26);
        assert_eq!(count_lantern_fishes(input, 80), 5934);
        assert_eq!(count_lantern_fishes(input, 256), 26984457539);
    }
}
