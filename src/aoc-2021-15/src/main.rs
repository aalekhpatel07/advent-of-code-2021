use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io;
use std::io::Read;

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Couldn't read data from stdin.");
    let grid: Grid = parser::read_grid(&buffer).expect("Couldn't parse grid from input data.");

    let result_1: usize = part_1::solve(&grid);
    let result_2: usize = part_2::solve(&grid);

    println!("Part 1: {:?}\nPart 2: {:?}", result_1, result_2);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid(Vec<Vec<usize>>);

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord(usize, usize);

#[derive(Copy, Debug, PartialEq, Eq, Clone)]
pub struct SearchCandidate(usize, Coord);

impl Ord for SearchCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for SearchCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

mod parser {
    use super::Grid;
    use std::io::ErrorKind;

    pub fn read_grid(input: &str) -> Result<Grid, ErrorKind> {
        Ok(Grid(
            input
                .split_whitespace()
                .map(|x| {
                    x.chars()
                        .map(|c| c.to_string().parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        ))
    }
}

pub trait Cost {
    fn cost(&self, c: Coord) -> usize;
    fn heuristic(&self, c1: Coord, c2: Coord) -> usize;
}

impl Cost for Grid {
    fn cost(&self, c: Coord) -> usize {
        *self.0.get(c.0).unwrap().get(c.1).unwrap()
    }

    fn heuristic(&self, c1: Coord, c2: Coord) -> usize {
        let mut sum: usize = 0;
        if c1.0 >= c2.0 {
            sum += c1.0 - c2.0;
        } else {
            sum += c2.0 - c1.0;
        }
        if c1.1 >= c2.1 {
            sum += c1.1 - c2.1;
        } else {
            sum += c2.1 - c1.1;
        }
        sum
    }
}

pub fn a_star_shortest_path<T>(
    data_structure: &T,
    start: Coord,
    end: Coord,
    rows: usize,
    cols: usize,
) -> Vec<Coord>
where
    T: Cost,
{
    let mut queue: BinaryHeap<SearchCandidate> = BinaryHeap::new();
    queue.push(SearchCandidate(data_structure.cost(start), start));

    let mut came_from: HashMap<Coord, Option<Coord>> = HashMap::new();
    let mut cost_so_far: HashMap<Coord, usize> = HashMap::new();

    came_from.insert(start, None);
    cost_so_far.insert(start, data_structure.cost(start));

    while !queue.is_empty() {
        let search_candidate = queue.pop().unwrap();
        let current_coord = search_candidate.1;

        if current_coord == end {
            break;
        }

        let mut neighbors: HashSet<Coord> = HashSet::new();
        for (shift_x, shift_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (candidate_x, candidate_y) = (
                current_coord.0 as i32 + shift_x,
                current_coord.1 as i32 + shift_y,
            );

            if 0 <= candidate_x
                && candidate_x < rows as i32
                && 0 <= candidate_y
                && candidate_y < cols as i32
            {
                neighbors.insert(Coord(candidate_x as usize, candidate_y as usize));
            }
        }

        for neighbor in neighbors {
            let new_cost: usize =
                *cost_so_far.get(&current_coord).unwrap() + data_structure.cost(neighbor);
            if !cost_so_far.contains_key(&neighbor)
                || new_cost < *cost_so_far.get(&neighbor).unwrap()
            {
                cost_so_far.insert(neighbor, new_cost);
                queue.push(SearchCandidate(
                    new_cost + data_structure.heuristic(end, neighbor),
                    neighbor,
                ));
                came_from.insert(neighbor, Some(current_coord));
            }
        }
    }

    let mut result: Vec<Coord> = vec![];

    let mut parent: Coord = end;

    while let Some(par) = *came_from.get(&parent).unwrap() {
        result.insert(0, parent);
        parent = par;
    }

    result.insert(0, start);
    result
}

mod part_1 {
    use super::{a_star_shortest_path, Coord, Grid};

    pub fn solve(grid: &Grid) -> usize {
        let rows = grid.0.len();
        let cols = grid.0.get(0).unwrap().len();

        let start: Coord = Coord(0, 0);
        let end: Coord = Coord(rows - 1, cols - 1);

        let shortest_path = a_star_shortest_path(grid, start, end, rows, cols);

        let weights = shortest_path
            .iter()
            .map(|&c| *grid.0.get(c.0).unwrap().get(c.1).unwrap())
            .collect::<Vec<usize>>();

        weights.iter().sum::<usize>() - weights[0]
    }
}

mod part_2 {
    use super::{a_star_shortest_path, Coord, Grid};

    pub fn expand_grid(grid: &Grid, scale: usize) -> Grid {
        let rows = grid.0.len();
        let cols = grid.0.get(0).unwrap().len();

        // Probably memory intensive for large sizes.
        // But works for AoC so... why not?
        let mut result: Vec<Vec<usize>> = vec![vec![0; cols * scale]; rows * scale];

        for row_block_idx in 0..scale {
            for col_block_idx in 0..scale {
                let row_offset = rows * row_block_idx;
                let col_offset = cols * col_block_idx;

                for row_idx in 0..rows {
                    for col_idx in 0..cols {
                        let value: usize = *grid.0.get(row_idx).unwrap().get(col_idx).unwrap();
                        let updated: usize = {
                            let raw = (value + (row_block_idx + col_block_idx)) % 9;
                            if raw == 0 {
                                9
                            } else {
                                raw
                            }
                        };
                        *result
                            .get_mut(row_offset + row_idx)
                            .unwrap()
                            .get_mut(col_offset + col_idx)
                            .unwrap() = updated;
                    }
                }
            }
        }
        Grid(result)
    }

    pub fn solve(grid: &Grid) -> usize {
        let expanded: Grid = expand_grid(grid, 5);

        let rows = expanded.0.len();
        let cols = expanded.0.get(0).unwrap().len();

        let start: Coord = Coord(0, 0);
        let end: Coord = Coord(rows - 1, cols - 1);

        let shortest_path = a_star_shortest_path(&expanded, start, end, rows, cols);

        let weights = shortest_path
            .iter()
            .map(|&c| *expanded.0.get(c.0).unwrap().get(c.1).unwrap())
            .collect::<Vec<usize>>();

        weights.iter().sum::<usize>() - weights[0]
    }
}

#[cfg(test)]
mod tests {
    use super::{parser, part_1, part_2, Grid};

    fn setup() -> Grid {
        let input: &str = "
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        ";
        parser::read_grid(input).expect("Couldn't parse sample grid.")
    }

    #[test]
    fn test_sample_1() {
        let data: Grid = setup();
        let expected: usize = 40;
        let observed: usize = part_1::solve(&data);
        assert_eq!(expected, observed);
    }
    #[test]
    fn test_sample_2() {
        let data: Grid = setup();
        let expected: usize = 315;
        let observed: usize = part_2::solve(&data);
        assert_eq!(expected, observed);
    }

    #[test]
    fn test_expand_grid() {
        let grid: Grid = setup();
        let observed = part_2::expand_grid(&grid, 5);
        let expanded_input: &str = "
        11637517422274862853338597396444961841755517295286
        13813736722492484783351359589446246169155735727126
        21365113283247622439435873354154698446526571955763
        36949315694715142671582625378269373648937148475914
        74634171118574528222968563933317967414442817852555
        13191281372421239248353234135946434524615754563572
        13599124212461123532357223464346833457545794456865
        31254216394236532741534764385264587549637569865174
        12931385212314249632342535174345364628545647573965
        23119445813422155692453326671356443778246755488935
        22748628533385973964449618417555172952866628316397
        24924847833513595894462461691557357271266846838237
        32476224394358733541546984465265719557637682166874
        47151426715826253782693736489371484759148259586125
        85745282229685639333179674144428178525553928963666
        24212392483532341359464345246157545635726865674683
        24611235323572234643468334575457944568656815567976
        42365327415347643852645875496375698651748671976285
        23142496323425351743453646285456475739656758684176
        34221556924533266713564437782467554889357866599146
        33859739644496184175551729528666283163977739427418
        35135958944624616915573572712668468382377957949348
        43587335415469844652657195576376821668748793277985
        58262537826937364893714847591482595861259361697236
        96856393331796741444281785255539289636664139174777
        35323413594643452461575456357268656746837976785794
        35722346434683345754579445686568155679767926678187
        53476438526458754963756986517486719762859782187396
        34253517434536462854564757396567586841767869795287
        45332667135644377824675548893578665991468977611257
        44961841755517295286662831639777394274188841538529
        46246169155735727126684683823779579493488168151459
        54698446526571955763768216687487932779859814388196
        69373648937148475914825958612593616972361472718347
        17967414442817852555392896366641391747775241285888
        46434524615754563572686567468379767857948187896815
        46833457545794456865681556797679266781878137789298
        64587549637569865174867197628597821873961893298417
        45364628545647573965675868417678697952878971816398
        56443778246755488935786659914689776112579188722368
        55172952866628316397773942741888415385299952649631
        57357271266846838237795794934881681514599279262561
        65719557637682166874879327798598143881961925499217
        71484759148259586125936169723614727183472583829458
        28178525553928963666413917477752412858886352396999
        57545635726865674683797678579481878968159298917926
        57944568656815567976792667818781377892989248891319
        75698651748671976285978218739618932984172914319528
        56475739656758684176786979528789718163989182927419
        67554889357866599146897761125791887223681299833479
        ";

        let expected: Grid =
            parser::read_grid(expanded_input).expect("Couldn't parse expanded grid.");
        assert_eq!(observed, expected);
    }
}
