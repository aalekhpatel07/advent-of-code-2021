use std::io;

fn main() {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap_or_else(|_| panic!("No input provided."));

    let fishes: Vec<usize> = {
        buffer
            .trim()
            .split(',')
            .map(|x| {
                x.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Couldn't parse: {:?}", x))
            })
            .collect::<Vec<usize>>()
    };

    buffer.clear();

    let result_part_1: usize = part_1::solve(&fishes);
    let result_part_2: usize = part_2::solve(&fishes);

    println!("Part 1: {:?}\nPart 2: {:?}", result_part_1, result_part_2);
}

pub fn step(counts: &[usize; 9]) -> [usize; 9] {
    [
        counts[1],             // all 0s come from 1s.
        counts[2],             // all 1s come from 2s.
        counts[3],             // all 2s come from 3s.
        counts[4],             // all 3s come from 4s.
        counts[5],             // all 4s come from 5s.
        counts[6],             // all 5s come from 6s.
        counts[0] + counts[7], // all 6s come from 0s and 7s
        counts[8],             // all 7s come from 8s.
        counts[0],             // all 8s come from 0s.
    ]
}

mod part_1 {
    use super::step;

    pub fn solve(fishes: &[usize]) -> usize {
        let mut counts: [usize; 9] = [0; 9];
        const TARGET_DAYS: usize = 80;

        fishes.iter().for_each(|&fish| {
            counts[fish - 1] += 1;
        });

        let mut result: [usize; 9] = counts;
        for _ in 0..(TARGET_DAYS - 1) {
            result = step(&result);
        }
        result.iter().sum::<usize>()
    }
}

mod part_2 {
    use super::step;

    pub fn solve(fishes: &[usize]) -> usize {
        let mut counts: [usize; 9] = [0; 9];
        const TARGET_DAYS: usize = 256;

        fishes.iter().for_each(|&fish| {
            counts[fish - 1] += 1;
        });

        let mut result: [usize; 9] = counts;

        for _ in 0..(TARGET_DAYS - 1) {
            result = step(&result);
        }
        result.iter().sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let fishes: Vec<usize> = vec![3, 4, 3, 1, 2];
        let expected: usize = 5_934;
        let observed: usize = part_1::solve(&fishes);
        assert_eq!(expected, observed);
    }

    #[test]
    fn test_sample_2() {
        let fishes: Vec<usize> = vec![3, 4, 3, 1, 2];
        let expected: usize = 26_984_457_539;
        let observed: usize = part_2::solve(&fishes);
        assert_eq!(expected, observed);
    }
}
