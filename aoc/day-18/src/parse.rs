use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map,
    sequence::tuple,
    *,
};

use crate::{Pair, SnailFish};

pub trait Parse {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for Pair {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, b, _, _, _, d, _, _)) = tuple((
            tag("["),
            space0,
            SnailFish::parse,
            space0,
            tag(","),
            space0,
            SnailFish::parse,
            space0,
            tag("]"),
        ))(s)?;

        Ok((s, Pair { left: b, right: d }))
    }
}

impl Parse for SnailFish {
    fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(digit1, |s: &str| {
                SnailFish::Literal(s.parse::<usize>().unwrap())
            }),
            map(Pair::parse, |p| SnailFish::Pair(Box::new(p))),
        ))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn simple_parse() {
        let raw = "[1,2]";
        let (_, p) = Pair::parse(raw).unwrap();
        assert_eq!(p.left, SnailFish::Literal(1));
        assert_eq!(p.right, SnailFish::Literal(2));
    }
    #[test]
    fn simple_parse2() {
        let raw = "[[1,2],3]";
        let (_, p) = Pair::parse(raw).unwrap();
        assert_eq!(
            p.left,
            SnailFish::Pair(Box::new(Pair {
                left: SnailFish::Literal(1),
                right: SnailFish::Literal(2)
            }))
        );
        assert_eq!(p.right, SnailFish::Literal(3));
    }

    #[test]
    fn simple_parse3() {
        let raw = "[[[[[9,8],1],2],3],4]";
        let (_, p) = Pair::parse(raw).unwrap();
        assert_eq!(p.right, SnailFish::Literal(4));
    }
}
