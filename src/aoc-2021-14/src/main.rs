use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::Read;

type Pair = (char, char);

#[derive(Debug, Clone)]
pub struct Counter<T> {
    pub inner: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Copy + Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut usize> {
        self.inner.get_mut(key)
    }

    pub fn get(&mut self, key: &T) -> Option<&usize> {
        self.inner.get(key)
    }
}

impl<U> FromIterator<U> for Counter<U>
where
    U: Copy + Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let mut counter: Counter<U> = Counter::new();

        for i in iter {
            if let std::collections::hash_map::Entry::Vacant(e) = counter.inner.entry(i) {
                e.insert(1);
            } else {
                let current_count = counter.inner.get_mut(&i).unwrap();
                *current_count += 1;
            }
        }
        counter
    }
}

impl<T> Default for Counter<T>
where
    T: Copy + Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut buffer: String = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Couldn't read from stdin.");

    let mut split = buffer.split('\n');
    let polymer = split.next().unwrap();
    split.next();

    let mut mapping: HashMap<Pair, char> = HashMap::new();

    for buf in split {
        if buf.is_empty() {
            break;
        }

        #[allow(clippy::iter_nth_zero)]
        let first: char = buf.chars().nth(0).unwrap();
        #[allow(clippy::iter_nth_zero)]
        let second: char = buf.chars().nth(1).unwrap();

        let last: char = buf.chars().nth(buf.len() - 1).unwrap();
        mapping.insert((first, second), last);
    }

    let result_1 = part_1::solve(polymer, &mapping);
    let result_2 = part_2::solve(polymer, &mapping);
    println!("Part 1: {:?}\nPart 2: {:?}", result_1, result_2);
}

pub fn step(
    pair_counter: &Counter<Pair>,
    individual_counter: &mut Counter<char>,
    mappings: &HashMap<Pair, char>,
) -> Counter<Pair> {
    let mut updated_pair_counter = Counter::<Pair>::new();

    for (pair, count) in &pair_counter.inner {
        if mappings.contains_key(pair) {
            let new_char: char = *mappings.get(pair).unwrap();

            if let std::collections::hash_map::Entry::Vacant(e) =
                individual_counter.inner.entry(new_char)
            {
                e.insert(*count);
            } else {
                let val = individual_counter.get_mut(&new_char).unwrap();
                *val += count;
            }

            let pair_to_update: Pair = (pair.0, new_char);
            if let std::collections::hash_map::Entry::Vacant(e) =
                updated_pair_counter.inner.entry(pair_to_update)
            {
                e.insert(*count);
            } else {
                let val = updated_pair_counter.get_mut(&pair_to_update).unwrap();
                *val += *count;
            }

            let pair_to_update: Pair = (new_char, pair.1);
            if let std::collections::hash_map::Entry::Vacant(e) =
                updated_pair_counter.inner.entry(pair_to_update)
            {
                e.insert(*count);
            } else {
                let val = updated_pair_counter.get_mut(&pair_to_update).unwrap();
                *val += *count;
            }
        } else {
            panic!("No mapping found!? Probably malformed input.");
        }
    }

    updated_pair_counter
}

mod part_1 {
    use super::{step, Counter, Pair};
    use std::collections::HashMap;

    pub fn solve(word: &str, mapping: &HashMap<Pair, char>) -> usize {
        let mut pair_counter: Counter<Pair> =
            Counter::from_iter(word.chars().zip(word.chars().skip(1)));
        let mut individual_counter: Counter<char> = Counter::from_iter(word.chars());

        for _ in 0..10usize {
            pair_counter = step(&pair_counter, &mut individual_counter, mapping);
        }
        individual_counter.inner.values().max().unwrap()
            - individual_counter.inner.values().min().unwrap()
    }
}
mod part_2 {
    use super::{step, Counter, Pair};
    use std::collections::HashMap;

    pub fn solve(word: &str, mapping: &HashMap<Pair, char>) -> usize {
        let mut pair_counter: Counter<Pair> =
            Counter::from_iter(word.chars().zip(word.chars().skip(1)));
        let mut individual_counter: Counter<char> = Counter::from_iter(word.chars());

        for _ in 0..40usize {
            pair_counter = step(&pair_counter, &mut individual_counter, mapping);
        }
        individual_counter.inner.values().max().unwrap()
            - individual_counter.inner.values().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{part_1, part_2, Counter};

    #[test]
    pub fn test_counter_new() {
        let counter: Counter<&str> = Counter::new();
        assert_eq!(counter.inner.len(), 0);
    }

    #[test]
    pub fn test_counter_from_str() {
        let counter: Counter<&str> = Counter::from_iter(["abc", "abc", "def"]);
        assert_eq!(counter.inner.len(), 2);
        assert_eq!(counter.inner.get(&"abc").unwrap(), &2);
        assert_eq!(counter.inner.get(&"def").unwrap(), &1);
    }

    #[test]
    pub fn test_counter_from_usize() {
        let counter: Counter<usize> = Counter::from_iter([0, 1, 2, 1, 1, 2, 1, 2]);
        assert_eq!(counter.inner.len(), 3);
        assert_eq!(counter.inner.get(&0).unwrap(), &1);
        assert_eq!(counter.inner.get(&1).unwrap(), &4);
        assert_eq!(counter.inner.get(&2).unwrap(), &3);
        assert_eq!(counter.inner.get(&3), None);
    }

    fn setup() -> (&'static str, HashMap<Pair, char>) {
        let input: &str = "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        let mut split = input.split('\n');
        split.next();
        let polymer = split.next().unwrap().trim();
        split.next();

        let mut mapping: HashMap<Pair, char> = HashMap::new();

        for buf in split {
            if buf.trim().is_empty() {
                break;
            }

            #[allow(clippy::iter_nth_zero)]
            let first: char = buf.trim().chars().nth(0).unwrap();
            #[allow(clippy::iter_nth_zero)]
            let second: char = buf.trim().chars().nth(1).unwrap();

            let last: char = buf.trim().chars().nth(buf.trim().len() - 1).unwrap();
            mapping.insert((first, second), last);
        }
        (polymer, mapping)
    }

    #[test]
    pub fn test_sample_1() {
        let (polymer, mapping) = setup();
        let observed = part_1::solve(polymer, &mapping);
        let expected: usize = 1588;
        assert_eq!(observed, expected);
    }
    #[test]
    pub fn test_sample_2() {
        let (polymer, mapping) = setup();
        let observed = part_2::solve(polymer, &mapping);
        let expected: usize = 2188189693529;
        assert_eq!(observed, expected);
    }
}
