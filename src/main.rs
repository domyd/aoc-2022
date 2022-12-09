use std::env;

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string("src/input/example.txt").unwrap();
    let result = match args.into_iter().skip(1).next().map(|n| n.parse::<u32>()) {
        Some(Ok(1)) => days::day09::one(&input),
        Some(Ok(2)) => days::day09::two(&input),
        _ => {
            eprintln!("Must pass either `1` or `2` as first argument.");
            return;
        }
    };

    eprintln!("{}", result);
}
