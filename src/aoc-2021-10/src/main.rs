use std::io;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub enum ScopeType {
    Bracket,
    Parenthesis,
    Brace,
    Angle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScopeSymbol {
    pub kind: ScopeType,
    pub symbol: char,
    pub complement: char,
    pub is_opening: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorStructure {
    Corrupted,
    Incomplete,
    UnknownSymbol,
    NoOpeningPair,
}

#[derive(Debug, Clone)]
pub struct LineParseError<'a> {
    pub source: &'a str,
    pub index: Option<usize>,
    pub expected: Option<char>,
    pub observed: Option<char>,
    pub symbol: Option<ScopeSymbol>,
    pub kind: ErrorStructure,
}

impl<'a> ScopeSymbol {
    fn new(kind: ScopeType, symbol: char, complement: char, is_opening: bool) -> Self {
        Self {
            kind,
            symbol,
            complement,
            is_opening,
        }
    }
    pub fn try_from_char(symbol: char) -> Result<Self, LineParseError<'a>> {
        match symbol {
            '(' => Ok(Self::new(ScopeType::Parenthesis, symbol, ')', true)),
            ')' => Ok(Self::new(ScopeType::Parenthesis, symbol, '(', false)),
            '[' => Ok(Self::new(ScopeType::Bracket, symbol, ']', true)),
            ']' => Ok(Self::new(ScopeType::Bracket, symbol, '[', false)),
            '{' => Ok(Self::new(ScopeType::Brace, symbol, '}', true)),
            '}' => Ok(Self::new(ScopeType::Brace, symbol, '{', false)),
            '<' => Ok(Self::new(ScopeType::Angle, symbol, '>', true)),
            '>' => Ok(Self::new(ScopeType::Angle, symbol, '<', false)),
            _ => Err(LineParseError {
                source: "",
                index: None,
                expected: None,
                observed: Some(symbol),
                symbol: None,
                kind: ErrorStructure::UnknownSymbol,
            }),
        }
    }
}

pub fn parse_line(input: &str) -> Result<Vec<ScopeSymbol>, LineParseError> {
    let mut stack: Vec<ScopeSymbol> = vec![];

    for (idx, c) in input.chars().enumerate() {
        match ScopeSymbol::try_from_char(c) {
            Ok(scope_symbol) => match scope_symbol.is_opening {
                true => {
                    stack.push(scope_symbol);
                }
                false => {
                    if let Some(last) = stack.last() {
                        if last.is_opening && scope_symbol.complement == last.symbol {
                            stack.pop();
                        } else {
                            return Err(LineParseError {
                                source: input,
                                index: Some(idx),
                                expected: Some(last.complement),
                                observed: Some(scope_symbol.symbol),
                                symbol: Some(scope_symbol),
                                kind: ErrorStructure::Corrupted,
                            });
                        }
                    } else {
                        return Err(LineParseError {
                            source: input,
                            index: Some(idx),
                            expected: None,
                            observed: Some(c),
                            symbol: Some(scope_symbol),
                            kind: ErrorStructure::NoOpeningPair,
                        });
                    }
                }
            },
            Err(e) => {
                return Err(LineParseError {
                    source: input,
                    index: Some(idx),
                    expected: None,
                    observed: e.observed,
                    symbol: None,
                    kind: ErrorStructure::UnknownSymbol,
                });
            }
        }
    }
    Ok(stack)
}

mod part_1 {
    use super::{parse_line, ErrorStructure, ScopeType};

    fn score(scope_type: ScopeType) -> usize {
        match scope_type {
            ScopeType::Parenthesis => 3,
            ScopeType::Bracket => 57,
            ScopeType::Brace => 1197,
            ScopeType::Angle => 25137,
        }
    }

    pub fn solve(inputs: &[&str]) -> usize {
        inputs
            .iter()
            .filter_map(|&inp| parse_line(inp).err()) // Keep only the errors.
            .filter(|err| err.kind == ErrorStructure::Corrupted) // Only those that are corrupted.
            .map(|err| score(err.symbol.unwrap().kind)) // Find the scores.
            .sum()
    }
}

mod part_2 {
    use super::{parse_line, ScopeType};

    fn score(scope_type: ScopeType) -> usize {
        match scope_type {
            ScopeType::Parenthesis => 1,
            ScopeType::Bracket => 2,
            ScopeType::Brace => 3,
            ScopeType::Angle => 4,
        }
    }

    pub fn solve(inputs: &[&str]) -> usize {
        let mut scores = inputs
            .iter()
            .filter_map(|&inp| parse_line(inp).ok()) // Keep only the valid results.
            .map(|stack| {
                stack
                    .iter()
                    .rev()
                    .fold(0usize, |acc, x| 5 * acc + score(x.kind.clone()))
            })
            .collect::<Vec<usize>>();

        scores.sort_unstable();
        *scores.get(scores.len() / 2).unwrap()
    }
}

fn main() {
    let mut buffer: String = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let inputs: Vec<&str> = buffer.split_whitespace().collect::<Vec<&str>>();

    let result_part_1: usize = part_1::solve(&inputs);
    let result_part_2: usize = part_2::solve(&inputs);

    println!("Part 1: {:?}\nPart 2: {:?}", result_part_1, result_part_2);
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    fn setup() -> Vec<&'static str> {
        let input: &'static str = "
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        ";
        input.split_whitespace().collect()
    }

    #[test]
    fn test_sample_1() {
        let clean = setup();
        assert_eq!(26397, part_1::solve(&clean));
    }
    #[test]
    fn test_sample_2() {
        let clean = setup();
        assert_eq!(288957, part_2::solve(&clean));
    }
}
