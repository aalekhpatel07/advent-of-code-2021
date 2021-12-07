use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Read};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bingo {
    pub(crate) entries: Vec<Vec<usize>>,
    value_idx_map: HashMap<usize, (usize, usize)>,
    played: [[bool; 5]; 5],
}

impl Default for Bingo {
    fn default() -> Self {
        Self::new()
    }
}

impl Bingo {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            value_idx_map: HashMap::<usize, (usize, usize)>::new(),
            played: [[false; 5]; 5],
        }
    }

    pub fn play(&mut self, number: usize) {
        let indices = self.value_idx_map.get(&number);
        if let Some((row_idx, col_idx)) = indices {
            self.played[*row_idx][*col_idx] = true;
        }
    }

    pub fn unplayed_sum(&self) -> usize {
        let mut result: usize = 0;

        for row_idx in 0..5 {
            for col_idx in 0..5 {
                if !self.played[row_idx][col_idx] {
                    result += self
                        .entries
                        .get(row_idx)
                        .unwrap_or_else(|| panic!("Row idx: {:?} not found.", row_idx))
                        .get(col_idx)
                        .unwrap_or_else(|| {
                            panic!("Row, Col idx: ({:?}, {:?}) not found.", row_idx, col_idx)
                        })
                }
            }
        }
        result
    }
    pub fn has_finished(&self) -> bool {
        // Check rows
        for row in self.played {
            // Looks weird but [true; 5] doesn't seem to work here.
            if row == [true; 5] {
                return true;
            }
        }

        // Check columns.
        for col_idx in 0..5 {
            let mut current_col: [bool; 5] = [false; 5];

            for (idx, item) in current_col.iter_mut().enumerate() {
                *item = self.played[idx][col_idx];
            }
            if current_col == [true; 5] {
                return true;
            }
        }
        false
    }

    pub fn parse<I: BufRead>(mut input: I) -> io::Result<Self> {
        let mut buffer: String = String::new();
        let mut whole_vec: Vec<Vec<usize>> = Vec::new();

        for _ in 0..6 {
            input.read_line(&mut buffer)?;
            let current_vec: Vec<usize> = buffer
                .split_whitespace()
                .map(|x| {
                    x.parse::<usize>()
                        .unwrap_or_else(|_| panic!("Could not parse: {:?}", x))
                })
                .collect::<Vec<usize>>();
            if current_vec.len() == 5 {
                whole_vec.push(current_vec);
            }
            buffer.clear();
        }

        let mut value_idx_map: HashMap<usize, (usize, usize)> = HashMap::new();

        for (row_idx, row) in whole_vec.iter().enumerate() {
            for (col_idx, &col) in row.iter().enumerate() {
                value_idx_map.insert(col, (row_idx, col_idx));
            }
        }

        Ok(Self {
            entries: whole_vec,
            value_idx_map,
            played: [[false; 5]; 5],
        })
    }

    pub fn parse_many<I: BufRead>(mut input: I) -> io::Result<Vec<Self>> {
        let mut buffer: String = String::new();
        input
            .read_to_string(&mut buffer)
            .expect("Couldn't read_to_string.");
        let mut lines = buffer.lines().collect::<Vec<&str>>();

        lines = lines
            .into_iter()
            .filter(|&x| x.split_whitespace().count() > 0)
            .collect::<Vec<&str>>();

        let mut bingos: Vec<Bingo> = Vec::new();

        if lines.len() % 5 != 0 {
            panic!("Not enough lines: given ({:?})", lines.len());
        }

        for chunk in &lines.into_iter().chunks(5) {
            let result = Self::parse(chunk.into_iter().join("\n").as_bytes());
            match result {
                Ok(bingo) => bingos.push(bingo),
                Err(e) => {
                    panic!("Wut? {:?}", e)
                }
            };
        }
        Ok(bingos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdin;

    #[test]
    fn test_bingo_create() {
        let mut b = Bingo::new();
        println!("{:?}", b);
    }

    #[test]
    fn test_bingo_parse() {
        let bingo_str = "
        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        ";
        let result = Bingo::parse(bingo_str.as_bytes()).unwrap();
        assert_eq!(
            [
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            result.entries.as_slice()
        );
    }

    #[test]
    fn test_multiple_bingo_parse() {
        let bingo_str = "
        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19
        ";

        let results =
            Bingo::parse_many(bingo_str.as_bytes()).expect("Couldn't parse multiple bingos.");

        assert_eq!(results.len(), 2);
        for result in results {
            assert_eq!(
                [
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ],
                result.entries.as_slice()
            );
        }
    }
}

fn parse_bingo_from_stdin() -> Result<(Vec<usize>, Vec<Bingo>), io::Error> {
    let mut buffer: String = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    let instructions_endpoint: usize = buffer
        .find('\n')
        .expect("Instructions should be separated by a newline.");
    let calls = &buffer[0..instructions_endpoint]
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("Couldn't parse {:?}", x))
        })
        .collect::<Vec<usize>>();
    let bingos: Vec<Bingo> = Bingo::parse_many(&mut buffer[instructions_endpoint + 1..].as_bytes())
        .expect("Some bingo couldn't be parsed.");

    Ok((calls.to_vec(), bingos))
}

mod part_1 {
    use super::Bingo;

    pub fn solve(games: &mut [Bingo], calls: &[usize]) -> usize {
        for &c in calls {
            for board in games.iter_mut() {
                board.play(c);
                if board.has_finished() {
                    return c * board.unplayed_sum();
                }
            }
        }
        0
    }
}

mod part_2 {
    use super::Bingo;
    use std::collections::HashSet;

    pub fn solve(games: &mut [Bingo], calls: &[usize]) -> usize {
        let games_len: usize = games.len();
        let mut last_winner: usize = usize::MAX;
        let mut already_won: HashSet<usize> = HashSet::new();
        let mut last_winning_call: usize = usize::MAX;

        for &c in calls {
            for (board_idx, board) in games.iter_mut().enumerate() {
                board.play(c);
                if board.has_finished() {
                    if !already_won.contains(&board_idx) {
                        already_won.insert(board_idx);
                        last_winner = board_idx;
                        last_winning_call = c;
                    }
                    if already_won.len() == games_len {
                        return c * games[board_idx].unplayed_sum();
                    }
                }
            }
        }
        last_winning_call * games[last_winner].unplayed_sum()
    }
}

fn main() {
    let (calls, mut games) = parse_bingo_from_stdin().expect("Parsing error.");
    let result_part_1 = part_1::solve(&mut games.clone(), &calls);
    let result_part_2 = part_2::solve(&mut games, &calls);

    println!("Part 1: {:}\nPart 2: {:}", result_part_1, result_part_2);
}
