use std::io;
use std::io::Read;


fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let grid = parser::read_grid(&buffer);

    let result_part_1 = part_1::solve(&grid);
    let result_part_2 = part_2::solve(&grid);

    println!("Part 1: {:?}\nPart 2: {:?}", result_part_1, result_part_2);

}


mod parser {
    fn read_row(inp: &str) -> Vec<usize> {
        inp
            .trim()
            .chars()
            .map(|x| String::from(x).parse::<usize>().unwrap_or_else(|_| panic!("Couldn't parse {:?}", x)))
            .collect::<Vec<usize>>()
    }

    pub fn read_grid(inp: &str) -> Vec<Vec<usize>> {
        inp
            .split_whitespace()
            .map(|x| read_row(x))
            .collect::<Vec<Vec<usize>>>()
    }

}


pub fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    if row == 0 {
        if col == 0 {
            vec![(row, col + 1), (row + 1, col)]
        } else if col < cols - 1 {
            vec![(row, col + 1), (row + 1, col), (row, col - 1)]
        } else {
            vec![(row + 1, col), (row, col - 1)]
        }
    } else if row < rows - 1 {
        if col == 0 {
            vec![(row, col + 1), (row + 1, col), (row - 1, col)]
        } else if col < cols - 1 {
            vec![(row, col + 1), (row + 1, col), (row, col - 1), (row - 1, col)]
        } else {
            vec![(row - 1, col), (row + 1, col), (row, col - 1)]
        }
    } else {
        if col == 0 {
            vec![(row, col + 1), (row - 1, col)]
        } else if col < cols - 1 {
            vec![(row, col + 1), (row, col - 1), (row - 1, col)]
        } else {
            vec![(row, col - 1), (row - 1, col)]
        }
    }
}


mod part_1 {
    use super::neighbors;

    fn is_sink(grid: &[Vec<usize>], neighbors: &[(usize, usize)], row: usize, col: usize) -> bool {

        let largest_neighbor: &(usize, usize) = neighbors
            .iter()
            .min_by_key(|(x, y)| Some(grid[*x][*y]))
            .unwrap();

        grid[largest_neighbor.0][largest_neighbor.1] > grid[row][col]
    }

    pub fn solve(grid: &[Vec<usize>]) -> usize {

        let rows: usize = grid.len();
        let cols: usize = grid[0].len();

        let mut low_points_score: usize = 0;

        for row in 0..rows {
            for col in 0..cols {
                if is_sink(grid, &neighbors(row, col, rows, cols), row, col) {
                    low_points_score += (grid[row][col] + 1);
                }
            }
        }
        low_points_score
    }
}


mod part_2 {
    use std::collections::BTreeSet;
    use std::error::Error;
    use std::hash::Hash;
    use super::neighbors;

    // It's weird that BTreeSet doesn't have a method like pop_random()
    // so let's define a custom one.
    fn pop<T>(set: &mut BTreeSet<T>) -> T
    where
        T: Clone + Eq + Hash + Ord
    {
        let mut it = set.iter();
        let first = it.next().cloned().unwrap();
        set.remove(&first);

        first
    }


    fn flood_fill(grid: &[Vec<usize>], wall: usize) -> Vec<BTreeSet<(usize, usize)>> {
        let rows: usize = grid.len();
        let cols: usize = grid[0].len();

        let mut unseen: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != wall {
                    unseen.insert((row, col));
                }
            }
        }

        let mut regions: Vec<BTreeSet<(usize, usize)>> = vec![];
        while !unseen.is_empty() {
            let (start_x, start_y) = pop(&mut unseen);
            let mut queue: Vec<(usize, usize)> = vec![(start_x, start_y)];
            let mut region: BTreeSet<(usize, usize)> = BTreeSet::new();

            while !queue.is_empty() {
                let (current_x, current_y) = queue.pop().unwrap();
                region.insert((current_x, current_y));

                for (shift_x, shift_y) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
                    let (mut neighbor_x, mut neighbor_y): (isize, isize) = (current_x as isize + shift_x, current_y as isize + shift_y);

                    if 0 <= neighbor_x.min(neighbor_y) && neighbor_x < rows as isize && neighbor_y < cols as isize {
                        if (
                            !region.contains(&(neighbor_x as usize, neighbor_y as usize)) &&
                            !seen.contains(&(neighbor_x as usize, neighbor_y as usize)) &&
                            grid[neighbor_x as usize][neighbor_y as usize] != wall
                        ) {
                            queue.push((neighbor_x as usize, neighbor_y as usize));
                        }
                    }
                }
            }

            for r in &region {
                unseen.remove(r);
            }
            regions.push(region.clone());

        }

        regions
    }

    pub fn solve(grid: &[Vec<usize>]) -> usize {
        let wall: usize = 9;
        let regions = flood_fill(grid, wall);

        let mut region_sizes: Vec<usize> = regions
            .iter()
            .map(|region| region.len())
            .collect::<Vec<usize>>();


        region_sizes.sort_unstable();

        let mut acc: usize = 1;
        for _ in 0..3 {
            acc *= region_sizes.pop().unwrap();
        }

        acc
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parse_grids() {
        let inp = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";

        let grid = parser::read_grid(inp);
        let rows = grid.len();
        let cols = grid[0].len();

        assert_eq!(rows, 5);
        assert_eq!(cols, 10);

        assert_eq!(grid, vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]);
    }

    #[test]
    fn test_sample_1() {

        let inp = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";

        let grid = parser::read_grid(inp);

        let result_part_1 = part_1::solve(&grid);
        assert_eq!(result_part_1, 15);
        // let result_part_2 = part_2::solve(&grid);
    }

    #[test]
    fn test_sample_2() {

        let inp = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";

        let grid = parser::read_grid(inp);

        let result_part_2 = part_2::solve(&grid);
        assert_eq!(result_part_2, 1134);
    }
}