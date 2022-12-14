use aoc_core::Solution;
use clap::Parser;
use clap::arg;
use clap::command;
use day_16;
use day_17;
use day_18;


#[derive(Debug, Parser)]
#[command(author = "Aalekh Patel", version = "0.1.0", about = "AOC driver.", long_about = "Run the solutions implemented in this workspace.")]
struct Args {
    #[arg(short, long, help="The day to compute solutions for.")]
    day: usize,
    #[arg(short, long, help="The part to solve. If provided, will solve part 2.", default_value_t = false)]
    second: bool
}

fn solve<S: Solution>(solver: S, second: bool) -> String {
    match second {
        true => solver.part2(),
        false => solver.part1()
    }
}


fn main() {
    let args = Args::parse();

    // Tried creating a Hashmap<usize, Box<dyn Solver>>
    // but the compiler is not happy and complains about
    // it unable to make the trait into an object because
    // it requires that Self: Sized. (I'm a bit lost here.)

    let answer: String = match args.day {
        16 => solve(day_16::Solver::default(), args.second),
        17 => solve(day_17::Solver::default(), args.second),
        18 => solve(day_18::Solver::default(), args.second),
        _ => unimplemented!("")
    };

    println!("Day: {}, Part: {}, Answer: {}", args.day, {if args.second {"2"} else {"1"}}, answer);
}
