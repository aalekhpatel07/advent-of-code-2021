use num_traits::Num;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;

#[derive(Debug, Clone, Eq)]
pub struct LineSegment<T>
where
    T: Num,
{
    pub start: Point<T>,
    pub end: Point<T>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Default for Point<T>
where
    T: Num,
{
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> Default for LineSegment<T>
where
    T: Num,
{
    fn default() -> Self {
        Self {
            start: Point::<T>::default(),
            end: Point::<T>::default(),
        }
    }
}

impl<T> Point<T>
where
    T: Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> LineSegment<T>
where
    T: Num,
{
    pub fn new((sx, sy): (T, T), (ex, ey): (T, T)) -> Self {
        Self {
            start: Point::new(sx, sy),
            end: Point::new(ex, ey),
        }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> PartialEq for LineSegment<T>
where
    T: Num,
{
    fn eq(&self, other: &Self) -> bool {
        ((other.start == self.start) && (other.end == self.end))
            || ((other.start == self.end) && (other.end == self.start))
    }
}

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::sequence::separated_pair;
    use nom::IResult;
    use std::io;

    use crate::{LineSegment, Point};

    pub fn point(input: &str) -> IResult<&str, Point<usize>> {
        let sp = separated_pair(
            nom::character::complete::u64,
            char(','),
            nom::character::complete::u64,
        )(input);

        match sp {
            Ok((remaining, nums)) => Ok((
                remaining,
                Point::<usize>::new(nums.0.try_into().unwrap(), nums.1.try_into().unwrap()),
            )),
            Err(e) => {
                println!("{:?}", input);
                Err(e)
            }
        }
    }

    pub fn line_segment(input: &str) -> Result<LineSegment<usize>, io::Error> {
        let segment = separated_pair(point, tag(" -> "), point)(input);

        match segment {
            Ok((remaining, nums)) => {
                if remaining.trim().is_empty() {
                    Ok(LineSegment::<usize>::new(
                        (nums.0.x, nums.0.y),
                        (nums.1.x, nums.1.y),
                    ))
                } else {
                    panic!("There's extra stuff in this line: {:?}", remaining.trim());
                }
            }
            Err(_) => {
                panic!("Couldn't parse line segment: {:?} ", input);
            }
        }
    }
}

fn incr<T>(hmap: &mut HashMap<Point<T>, T>, point: Point<T>)
where
    T: Num + Hash + Copy + Eq + From<usize>,
    // Copy makes this less powerful. There must be a better way.
{
    if let Some(count) = hmap.get_mut(&point) {
        *count = count.add(T::one());
    } else {
        hmap.insert(point, T::one());
    }
}

fn walk_left_to_right<T>(hmap: &mut HashMap<Point<T>, T>, left: Point<T>, right: Point<T>)
where
    T: Num + Copy + PartialOrd + Hash + PartialEq + Eq + From<usize>,
    usize: From<T>,
{
    if left.y < right.y {
        // top-left --> bottom-right
        let total_shift: usize = usize::try_from(right.y - left.y).unwrap();
        (0..=total_shift).for_each(|shift| {
            incr(
                hmap,
                Point::new(
                    left.x + T::try_from(shift).unwrap(),
                    left.y + T::try_from(shift).unwrap(),
                ),
            );
        });
    } else {
        // bottom-left --> top-right
        let total_shift: usize = usize::try_from(left.y - right.y).unwrap();
        (0..=total_shift).for_each(|shift| {
            incr(
                hmap,
                Point::new(
                    left.x + T::try_from(shift).unwrap(),
                    left.y - T::try_from(shift).unwrap(),
                ),
            );
        });
    }
}

pub mod part_1 {
    use super::{incr, LineSegment, Point};
    use std::collections::HashMap;

    pub fn solve(segments: &[LineSegment<usize>]) -> usize {
        let mut cover_map: HashMap<Point<usize>, usize> = HashMap::new();

        segments.iter().for_each(|segment| {
            if segment.start.x == segment.end.x {
                let upper = segment.start.y.max(segment.end.y);
                let lower = segment.start.y.min(segment.end.y);
                (lower..=upper).for_each(|cell| {
                    incr(&mut cover_map, Point::new(segment.start.x, cell));
                });
            } else if segment.start.y == segment.end.y {
                let upper = segment.start.x.max(segment.end.x);
                let lower = segment.start.x.min(segment.end.x);
                (lower..=upper).for_each(|cell| {
                    incr(&mut cover_map, Point::new(cell, segment.start.y));
                });
            }
        });
        cover_map.into_values().filter(|&x| x >= 2).count()
    }
}

pub mod part_2 {
    use super::{incr, walk_left_to_right, LineSegment, Point};
    use std::collections::HashMap;

    pub fn solve(segments: &[LineSegment<usize>]) -> usize {
        let mut cover_map: HashMap<Point<usize>, usize> = HashMap::new();

        segments.iter().for_each(|segment| {
            if segment.start.x == segment.end.x {
                let upper = segment.start.y.max(segment.end.y);
                let lower = segment.start.y.min(segment.end.y);
                (lower..=upper).for_each(|cell| {
                    incr(&mut cover_map, Point::new(segment.start.x, cell));
                });
            } else if segment.start.y == segment.end.y {
                let upper = segment.start.x.max(segment.end.x);
                let lower = segment.start.x.min(segment.end.x);
                (lower..=upper).for_each(|cell| {
                    incr(&mut cover_map, Point::new(cell, segment.start.y));
                });
            } else if segment.start.x > segment.end.x {
                walk_left_to_right(&mut cover_map, segment.end.clone(), segment.start.clone());
            } else {
                walk_left_to_right(&mut cover_map, segment.start.clone(), segment.end.clone());
            }
        });
        cover_map.into_values().filter(|&x| x >= 2).count()
    }
}

fn main() {
    let mut buffer: String = String::new();
    let mut line_segments: Vec<LineSegment<usize>> = vec![];
    while let Ok(size) = io::stdin().read_line(&mut buffer) {
        if size == 0 {
            break;
        }
        line_segments.push(
            parser::line_segment(&buffer)
                .unwrap_or_else(|_| panic!("Couldn't parse {:?} of {:?} bytes", buffer, size)),
        );
        buffer.clear();
    }

    let result_part_1 = part_1::solve(&line_segments);
    let result_part_2 = part_2::solve(&line_segments);

    println!("Part 1: {:}\nPart 2: {:}", result_part_1, result_part_2);
}

#[cfg(test)]
mod tests {
    use super::{parser, LineSegment, Point};
    use nom::IResult;

    #[test]
    fn test_default_point() {
        let p = Point::<f64>::default();
        println!("{:?}", p);
    }

    #[test]
    fn test_default_line_segment() {
        let ls = LineSegment::<f64>::default();
        println!("{:?}", ls);
    }

    #[test]
    fn test_parse_line_segment() {
        let input: &str = "0,9 -> 5,9";
        let observed = parser::line_segment(input);
        assert!(observed.is_ok());
        let expected = LineSegment::<usize>::new((0, 9), (5, 9));
        assert_eq!(observed.unwrap(), expected);
    }

    #[test]
    fn test_parse_point() {
        let input: &str = "5,9";
        let observed = parser::point(input);
        assert!(observed.is_ok());
        let expected = Point::<usize>::new(5, 9);
        assert_eq!(observed.unwrap().1, expected);
    }
}
