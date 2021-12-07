use std::io;

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin.");
    let numbers: Vec<usize> = {
        buffer
            .split(',')
            .map(|x| {
                x.trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Couldn't parse {:?} as an usize.", x))
            })
            .collect::<Vec<usize>>()
    };
    let result_part_1 = part1::solve(&numbers);
    let result_part_2 = part2::solve(&numbers);
    println!("Part 1: {:}\nPart 2: {:}", result_part_1, result_part_2);
}

pub mod part1 {
    use std::ops::Div;

    fn median(nums: &[usize]) -> usize {
        let mut cloned = nums.to_owned();
        cloned.sort_unstable();
        match nums.len() % 2 {
            0 => cloned[nums.len().div(2)],
            _ => (cloned[(nums.len() - 1).div(2)] + cloned[(nums.len() + 1).div(2)]).div(2),
        }
    }

    pub fn solve(nums: &[usize]) -> usize {
        let target: usize = median(nums);
        nums.iter()
            .map(|&x| if target >= x { target - x } else { x - target })
            .sum()
    }
}

pub mod part2 {

    pub fn solve(nums: &[usize]) -> usize {
        let &min = nums.iter().min().unwrap();
        let &max = nums.iter().max().unwrap();

        let mut minimum_cost: usize = usize::MAX;

        for candidate in min..max {
            let mut current_cost: usize = 0;
            for &num in nums {
                current_cost += {
                    if num >= candidate {
                        ((num - candidate) * ((num - candidate) + 1)) / 2
                    } else {
                        ((candidate - num) * ((candidate - num) + 1)) / 2
                    }
                }
            }
            minimum_cost = minimum_cost.min(current_cost);
        }
        minimum_cost
    }
}
