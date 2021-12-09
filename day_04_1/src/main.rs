use std::fmt::Display;
use std::str::FromStr;

use std::fs;

use regex::Regex;

use ansi_term::Style;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Square {
    Empty(u8),
    Marked(u8),
}

impl Square {
    fn is_marked(&self) -> bool {
        match self {
            Square::Marked(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Board {
    square: [[Square; 5]; 5],
}

impl Board {
    fn mark(&mut self, value: u8) -> bool {
        let mut marked = false;
        for i in 0..5 {
            for j in 0..5 {
                if let Square::Empty(v) = self.square[i][j] {
                    if v == value {
                        self.square[i][j] = Square::Marked(value);
                        marked = true;
                    }
                }
            }
        }
        marked
    }

    fn won(&self) -> bool {
        (0..5).any(|i| {
            self.col(i).iter().all(|s| s.is_marked()) || self.row(i).iter().all(|s| s.is_marked())
        })
    }

    fn score(&self, value: u8) -> u32 {
        self.square
            .iter()
            .map(|r| {
                r.iter()
                    .map(|s| match s {
                        Square::Marked(_) => 0u32,
                        Square::Empty(v) => (*v).into(),
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            * value as u32
    }

    fn col(&self, column_idx: usize) -> [Square; 5] {
        let mut col: [Square; 5] = [Square::Empty(0); 5];
        for i in 0..5 {
            col[i] = self.square[i][column_idx];
        }
        col
    }

    fn row(&self, row: usize) -> [Square; 5] {
        self.square[row]
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut square = [[Square::Empty(0); 5]; 5];
        for (i, line) in s.lines().enumerate() {
            for (j, v) in line
                .trim()
                .split(&Regex::new(r"[[:space:]]+").expect("Invalid regex"))
                .enumerate()
            {
                square[i][j] = v
                    .parse::<u8>()
                    .and_then(|v| Ok(Square::Empty(v)))
                    .or_else(|_| Err(()))?;
            }
        }
        Ok(Board { square })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                match self.square[i][j] {
                    Square::Empty(v) => write!(f, "{:2} ", v)?,
                    Square::Marked(v) => {
                        write!(f, "{} ", Style::new().bold().paint(format!("{:2}", v)))?
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    boards: Vec<Board>,
    turns: Vec<u8>,
    current: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TurnResult {
    Continue,
    Done,
    Win(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GameResult {
    Done,
    Won(u32),
}

impl Game {
    fn next_turn(&mut self) -> TurnResult {
        let draw = self.turns[self.current as usize];
        println!("Turn: {} - Draw: {}", self.current, draw);
        self.current += 1;
        if self.current == self.turns.len() as u32 {
            TurnResult::Done
        } else {
            for (i, b) in self.boards.iter_mut().enumerate() {
                if b.mark(draw) {
                    if b.won() {
                        return TurnResult::Win(b.score(draw));
                    }
                }
                println!("Board {}\n{}", i, b);
            }
            TurnResult::Continue
        }
    }

    fn play(&mut self) -> GameResult {
        loop {
            match self.next_turn() {
                TurnResult::Continue => {}
                TurnResult::Done => {
                    return GameResult::Done;
                }
                TurnResult::Win(s) => {
                    return GameResult::Won(s);
                }
            }
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut s = input.split("\n\n");
        let turns = s
            .next()
            .unwrap()
            .split(',')
            .map(|t| t.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let boards = s
            .map(|b| {
                println!("{}", b);
                b.parse::<Board>().unwrap()
            })
            .collect::<Vec<_>>();
        Ok(Game {
            boards,
            turns,
            current: 0,
        })
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut game = contents.parse::<Game>().unwrap();
    match game.play() {
        GameResult::Done => println!("Done, no winner :("),
        GameResult::Won(s) => println!("Won ! with score: {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_1: &str = "13

1 2 3 4 5
 6 7 8 9 10
11 12 13 14 15
16 17 18 19 20
21 22 23 24 25";

    const BOARD_1: &str = "1 2 3 4 5
6 7 8 9 10
11 12 13 14 15
16 17 18 19 20
21 22 23 24 25";

    const BOARD_1_PARSED: [[Square; 5]; 5] = [
        [
            Square::Empty(1),
            Square::Empty(2),
            Square::Empty(3),
            Square::Empty(4),
            Square::Empty(5),
        ],
        [
            Square::Empty(6),
            Square::Empty(7),
            Square::Empty(8),
            Square::Empty(9),
            Square::Empty(10),
        ],
        [
            Square::Empty(11),
            Square::Empty(12),
            Square::Empty(13),
            Square::Empty(14),
            Square::Empty(15),
        ],
        [
            Square::Empty(16),
            Square::Empty(17),
            Square::Empty(18),
            Square::Empty(19),
            Square::Empty(20),
        ],
        [
            Square::Empty(21),
            Square::Empty(22),
            Square::Empty(23),
            Square::Empty(24),
            Square::Empty(25),
        ],
    ];

    const BOARD_1_PARSED_MARKED: [[Square; 5]; 5] = [
        [
            Square::Empty(1),
            Square::Empty(2),
            Square::Empty(3),
            Square::Empty(4),
            Square::Empty(5),
        ],
        [
            Square::Empty(6),
            Square::Empty(7),
            Square::Empty(8),
            Square::Empty(9),
            Square::Empty(10),
        ],
        [
            Square::Empty(11),
            Square::Empty(12),
            Square::Marked(13),
            Square::Empty(14),
            Square::Empty(15),
        ],
        [
            Square::Empty(16),
            Square::Empty(17),
            Square::Empty(18),
            Square::Empty(19),
            Square::Empty(20),
        ],
        [
            Square::Empty(21),
            Square::Empty(22),
            Square::Empty(23),
            Square::Empty(24),
            Square::Empty(25),
        ],
    ];

    #[test]
    fn test_game_from_str() {
        let game = Game::from_str(GAME_1).unwrap();
        assert_eq!(
            game.boards,
            vec![Board {
                square: BOARD_1_PARSED
            }]
        );
        assert_eq!(game.turns, vec![13]);
    }

    #[test]
    fn test_won() {
        let mut board = Board::from_str(BOARD_1).unwrap();
        assert!(!board.won());
        board.mark(6);
        board.mark(7);
        board.mark(8);
        board.mark(9);
        board.mark(10);
        assert!(board.won());

        let mut board = Board::from_str(BOARD_1).unwrap();
        assert!(!board.won());
        board.mark(2);
        board.mark(7);
        board.mark(12);
        board.mark(17);
        board.mark(22);
        assert!(board.won());
    }

    #[test]
    fn test_board_from_str() {
        let mut board = Board::from_str(BOARD_1).unwrap();

        assert_eq!(board.square, BOARD_1_PARSED);

        board.mark(13);

        assert_eq!(board.square, BOARD_1_PARSED_MARKED);
    }

    const TEST: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_game() {
        let mut game = Game::from_str(TEST).unwrap();
        assert_eq!(game.boards.len(), 3);
        assert_eq!(game.turns.len(), 27);
        assert_eq!(game.current, 0);
        assert_eq!(game.play(), GameResult::Won(4512));
    }
}
