use std::{iter::IntoIterator, str::FromStr};

#[derive(Copy, Clone, Debug)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coord = s.split(',');
        let x = coord.next().unwrap().parse::<usize>().unwrap();
        let y = coord.next().unwrap().parse::<usize>().unwrap();
        Ok(Point(x as usize, y as usize))
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            start: self.start,
            end: self.end,
            current: 0,
            max: if (self.start.0 as i32 - self.end.0 as i32).abs()
                > (self.start.1 as i32 - self.end.1 as i32).abs()
            {
                (self.start.0 as i32 - self.end.0 as i32).abs() as usize
            } else {
                (self.start.1 as i32 - self.end.1 as i32).abs() as usize
            },
        }
    }
}

struct LineIterator {
    start: Point,
    end: Point,
    current: usize,
    max: usize,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            None
        } else {
            let y = if self.start.1 > self.end.1 {
                self.start.1 - self.current
            } else if self.start.1 < self.end.1 {
                self.start.1 + self.current
            } else {
                self.start.1
            };
            let x = if self.start.0 > self.end.0 {
                self.start.0 - self.current
            } else if self.start.0 < self.end.0 {
                self.start.0 + self.current
            } else {
                self.start.0
            };
            self.current += 1;
            Some(Point(x as usize, y as usize))
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" -> ");
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();
        let start = start.parse::<Point>().unwrap();
        let end = end.parse::<Point>().unwrap();
        if start.0 != end.0
            && start.1 != end.1
            && (start.0 as i32 - end.0 as i32).abs() != (start.1 as i32 - end.1 as i32).abs()
        {
            Err(())
        } else if start.0 > end.0 || start.1 > end.1 {
            Ok(Line {
                start: end,
                end: start,
            })
        } else {
            Ok(Line { start, end })
        }
    }
}

fn get_max_x(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|l| {
            if l.end.0 > l.start.0 {
                l.end.0
            } else {
                l.start.0
            }
        })
        .max()
        .unwrap()
}

fn get_max_y(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|l| {
            if l.end.1 > l.start.1 {
                l.end.1
            } else {
                l.start.1
            }
        })
        .max()
        .unwrap()
}

fn count_overlaps(input: &str) -> usize {
    let lines = input
        .lines()
        .filter_map(|l| l.parse::<Line>().ok())
        .collect::<Vec<_>>();
    let mut grid = vec![vec![0; get_max_x(&lines) + 1]; get_max_y(&lines) + 1];
    for line in &lines {
        for point in line.into_iter() {
            println!("{:?}", point);
            grid[point.1][point.0] += 1;
        }
    }
    grid.iter()
        .map(|row| row.iter().filter(|&&v| v > 1).count())
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Overlaps: {}", count_overlaps(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn it_works() {
        assert_eq!(count_overlaps(INPUT), 12);
    }
}
