use std::{ops::{Range, Add, AddAssign, Neg}};

use aoc_core::Solution;
use num_traits::Num;
use rayon::prelude::*;


#[derive(Copy, PartialEq, Eq, Debug)]
pub struct Vector<T>(T, T);


impl<T> Clone for Vector<T> where T: Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T> Add for Vector<T> where T: Add<Output = T> {
    type Output = Vector<T>;
    fn add(self, other: Vector<T>) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1)
    }
}


impl<T> AddAssign for Vector<T> where T: AddAssign<T> {
    fn add_assign(&mut self, other: Vector<T>) {
        self.0 += other.0;
        self.1 += other.1;
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState<T> {
    pub target_area: (Range<T>, Range<T>),
    pub initial_position: Vector<T>,
    pub initial_velocities: Vector<T>,
    pub position: Vector<T>,
    pub velocity: Vector<T>,
    pub largest_height: T,
}

impl<T: Number> GameState<T> {
    pub fn new(
        x_range: Range<T>, 
        y_range: Range<T>,
        initial_velocity_x: T,
        initial_velocity_y: T,
        initial_position_x: T,
        initial_position_y: T
    ) -> Self {
        Self {
            target_area: (x_range, y_range),
            initial_position: Vector::new(initial_position_x, initial_position_y),
            initial_velocities: Vector::new(initial_velocity_x, initial_velocity_y),
            position: Vector::new(initial_position_x, initial_position_y),
            velocity: Vector::new(initial_velocity_x, initial_velocity_y),
            largest_height: initial_position_y,
        }
    }

    pub fn is_in_target_area(&self) -> bool {
        self.target_area.0.contains(&self.position.0) && self.target_area.1.contains(&self.position.1)
    }

    pub fn completes_in_num_steps(&mut self, num_steps: usize) -> bool {
        for _ in 0..num_steps {
            self.step();
            if self.is_in_target_area() {
                return true;
            }
        }
        false
    }
}

pub struct Trajectory<T> {
    pub game_state: GameState<T>
}

impl<T: Number> Trajectory<T> {
    pub fn new(initial: GameState<T>) -> Self {
        Self { game_state: initial }
    }
}

impl<T: Number> Iterator for Trajectory<T> {
    type Item = Vector<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.game_state.is_in_target_area() {
            return None;
        }
        self.game_state.step();
        Some(self.game_state.position)
    }
}

impl<T: Number> IntoIterator for GameState<T> {
    type Item = Vector<T>;

    type IntoIter = Trajectory<T>;

    fn into_iter(self) -> Self::IntoIter {
        Trajectory::new(self)
    }
}



pub trait Number: Num + Clone + AddAssign + Copy + Neg<Output=Self> + PartialOrd + Ord {
    fn signum(value: Self) -> Self {
        if value > Self::zero() {
            Self::one()
        } else if value < Self::zero() {
            Self::one().neg()
        } else {
            Self::zero()
        }
    }
}
impl<T> Number for T where T: Num + Clone + AddAssign + Copy + Neg<Output=T> + PartialOrd + Ord {}

impl<T: Number> GameState<T> {
    pub fn step(&mut self) {
        self.position += self.velocity.clone();
        self.velocity += Vector::new(T::signum(self.velocity.0).neg(), T::one().neg());
        self.largest_height = self.largest_height.max(self.position.1);
    }
}

#[derive(Default)]
pub struct Solver {}

impl Solution for Solver {
    fn part1(&self) -> String {
        solve_part1()
    }
    fn part2(&self) -> String {
        solve_part2()
    }
}


pub fn solve_part1() -> String {
    let x_min = 0;
    let x_max = 1000;
    let y_min = 0;
    let y_max = 1000;
    let max_steps = 1000;

    let best = 
    (x_min..x_max).flat_map(|x| (y_min..y_max).clone().map(move |y| (x, y)))
    .collect::<Vec<(i32, i32)>>()
    // 24-core CPU goes brrrrr.
    .par_iter()
    .map(|&(ivx, ivy)| {
        let mut game_state = GameState::new(
            230..284, 
            -107..-56, 
            ivx, 
            ivy,
            0, 
            0
        );
        if game_state.completes_in_num_steps(max_steps) {
            game_state.largest_height
        } else {
            0
        }
    })
    .max()
    .unwrap();
    format!("{}", best)
}


pub fn solve_part2() -> String {
    // Some arbitrary range.
    let x_min = -1000;
    let x_max = 1000;
    let y_min = -1000;
    let y_max = 1000;
    let max_steps = 2_000;

    let best = 
    (x_min..x_max).flat_map(|x| (y_min..y_max).clone().map(move |y| (x, y)))
    .collect::<Vec<(i32, i32)>>()
    // 24-core CPU goes brrrrr.
    .par_iter()
    .map(|&(ivx, ivy)| {
        let mut game_state = GameState::new(
            230..284, 
            -107..-56, 
            ivx, 
            ivy,
            0, 
            0
        );
        game_state.completes_in_num_steps(max_steps)
    })
    .filter(|&v| v)
    .count();
    format!("{}", best)
}

#[cfg(test)]
mod tests {
    #[test]
    fn real() {
        assert_eq!(super::solve_part1(), "5671");
        assert_eq!(super::solve_part2(), "4556");
    }
}