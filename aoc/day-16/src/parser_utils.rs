use nom::{bytes::complete::take_while, combinator::map, IResult};

pub fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

pub fn is_binary(c: char) -> bool {
    c.is_digit(2)
}

pub fn is_zero(c: char) -> bool {
    c == '0'
}

pub fn is_one(c: char) -> bool {
    c == '1'
}

pub fn parse_hex(s: &str) -> IResult<&str, Vec<u8>> {
    map(take_while(is_hex_digit), |hex_str: &str| {
        hex_str
            .chars()
            .map(|c: char| u8::from_str_radix(c.to_string().as_str(), 16).unwrap())
            .collect::<Vec<u8>>()
    })(s)
}
