use std::collections::HashSet;
use std::io;
use std::io::Read;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub(crate) _inner: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid { _inner: vec![] }
    }

    pub fn incr(&mut self, value: usize) {
        for row in self._inner.iter_mut() {
            for entry in row.iter_mut() {
                *entry += value;
            }
        }
    }

    fn find_nines(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = vec![];

        for (row_idx, row) in self._inner.iter().enumerate() {
            for (col_idx, &entry) in row.iter().enumerate() {
                if entry > 9 {
                    positions.push((row_idx, col_idx));
                }
            }
        }
        positions
    }

    fn neighbors(&self, (row_idx, col_idx): (usize, usize)) -> Vec<(usize, usize)> {
        let mut coords: Vec<(usize, usize)> = vec![];

        for shift_vertical in -1_isize..=1_isize {
            for shift_horizontal in -1_isize..=1_isize {
                if shift_horizontal == 0 && shift_vertical == 0 {
                    continue;
                }
                let (cx, cy) = (
                    row_idx as isize + shift_vertical,
                    col_idx as isize + shift_horizontal,
                );
                if cx < 0 || cy < 0 {
                    continue;
                }
                let (cx, cy) = (cx as usize, cy as usize);

                if cx < self._inner.len() && cy < self._inner.get(0).unwrap().len() {
                    coords.push((cx, cy));
                }
            }
        }
        coords
    }

    pub fn start_flashes(&mut self) -> usize {
        let mut nines = self.find_nines();
        let mut total_flashes: usize = 0;
        let mut already_flashed: HashSet<(usize, usize)> = HashSet::new();

        while !nines.is_empty() {
            let mut neighbors_to_update: Vec<(usize, usize)> = vec![];

            for flasher in nines.iter() {
                if !already_flashed.contains(flasher) {
                    neighbors_to_update.extend(self.neighbors(*flasher));
                    already_flashed.insert(*flasher);
                    total_flashes += 1;
                }
            }
            if neighbors_to_update.is_empty() {
                break;
            }
            for (nx, ny) in neighbors_to_update {
                if !already_flashed.contains(&(nx, ny)) {
                    *self._inner.get_mut(nx).unwrap().get_mut(ny).unwrap() += 1;
                }
            }

            for (flasher_x, flasher_y) in nines {
                *self
                    ._inner
                    .get_mut(flasher_x)
                    .unwrap()
                    .get_mut(flasher_y)
                    .unwrap() = 0;
            }

            nines = self.find_nines();
        }

        total_flashes
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new()
    }
}

trait Step {
    fn step(&mut self) -> usize;
}

impl Step for Grid {
    fn step(&mut self) -> usize {
        self.incr(1);
        self.start_flashes()
    }
}

mod parser {
    use super::Grid;

    pub fn grid(input: &str) -> Grid {
        let grid_raw = input
            .split_whitespace()
            .map(|x| {
                x.chars()
                    .map(|c| String::from(c).parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();
        Grid { _inner: grid_raw }
    }
}

mod part_1 {
    use super::{Grid, Step};

    pub fn solve(grid: &mut Grid) -> usize {
        const STEPS: usize = 100;
        let mut acc: usize = 0;

        for _ in 0..STEPS {
            acc += grid.step();
        }
        acc
    }
}

mod part_2 {
    use super::{Grid, Step};

    pub fn solve(grid: &mut Grid) -> usize {
        let mut steps_taken: usize = 0;

        loop {
            if grid
                ._inner
                .iter()
                .map(|x| x.iter().sum::<usize>())
                .sum::<usize>()
                == 0
            {
                break;
            }
            grid.step();
            steps_taken += 1;
        }
        steps_taken
    }
}

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Could not read from stdin.");

    let mut grid: Grid = parser::grid(&buffer);

    let result_part_1 = part_1::solve(&mut grid.clone());
    let result_part_2 = part_2::solve(&mut grid);

    println!("Part 1: {:?}\nPart 2: {:?}", result_part_1, result_part_2);
}

#[cfg(test)]
mod tests {
    use super::{parser, part_1, part_2, Grid};

    fn setup() -> Grid {
        let input: &str = "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ";
        parser::grid(input)
    }

    #[test]
    fn test_sample_1() {
        let mut input = setup();
        assert_eq!(1656, part_1::solve(&mut input));
    }

    #[test]
    fn test_sample_2() {
        let mut input = setup();
        assert_eq!(195, part_2::solve(&mut input));
    }
}
