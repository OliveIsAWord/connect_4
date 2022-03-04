mod eval;
mod game;
mod play;
mod transpositions;

use eval::{Evaluator, Strength};
use game::{Position, Score};

use std::env;
use std::fs;
use std::time::SystemTime;

#[derive(Debug)]
struct Test {
    moves: String,
    eval: Score,
}

fn get_tests(filepath: &str) -> Vec<Test> {
    let raw = fs::read_to_string(filepath).unwrap();
    let lines = raw.lines();
    let mut tests = Vec::new(); // Lines does not have a `len`
    for line in lines {
        let idx = line.find(' ').unwrap();
        let moves = line[..idx].to_string();
        let eval = line[idx + 1..].parse().unwrap();
        tests.push(Test { moves, eval })
    }
    tests
}

fn main() {
    let mut fp: Option<String> = None;
    let mut test_strength = Strength::Strong;
    // Note: Consider using an existing CLI library such as Clap.
    // Alternately (and preferably), consider creating a new CLI library.
    for arg in env::args().skip(1) {
        if arg == "-w" || arg == "--weak" {
            match test_strength {
                Strength::Strong => test_strength = Strength::Weak,
                Strength::Weak => {
                    eprintln!("Error: Duplicate `weak` argument.");
                    return;
                }
            }
        } else if arg.starts_with('-') {
            eprintln!("Error: Unrecognized argument `{}`", arg);
            return;
        } else if fp.is_none() {
            fp = Some(arg);
        } else {
            eprintln!("Error: Unexpected argument `{}`", arg);
            return;
        }
    }
    let fp = match fp {
        Some(path) => path,
        None => {
            eprintln!("Error: No filepath for test provided.");
            return;
        }
    };
    println!("Getting tests from {}...", fp);
    let tests = get_tests(&fp);
    drop(fp);

    let mut time_per_test: Vec<u128> = Vec::with_capacity(tests.len());
    let mut num_fails = 0;
    println!("Initializing transposition table...");
    let mut my_eval = Evaluator::new(test_strength);
    println!("Running {} tests...", tests.len());
    for (i, test) in tests.iter().enumerate() {
        //println!("{:?}", test);
        let my_game = Position::from_moves(&test.moves);
        my_eval.reset();
        let now = SystemTime::now();
        let score = my_eval.eval(&my_game);
        match now.elapsed() {
            Ok(elapsed) => {
                let test_time = elapsed.as_micros();
                println!("{} Time taken: {}μs", i, test_time);
                //println!("Score: {}", score);
                time_per_test.push(test_time);
            }
            Err(e) => println!("Timing Error: {}", e),
        }

        let failed = match test_strength {
            Strength::Strong => score != test.eval,
            Strength::Weak => score != test.eval.signum(),
        };
        if failed {
            num_fails += 1;
            eprintln!("Test failed: {:?}", test);
            eprintln!("{}", my_game);
            eprintln!("Score:    {}", score);
            eprintln!("Expected: {}", test.eval);
        }
    }
    let total_time = time_per_test.iter().sum::<u128>();
    let mean_time = total_time as f64 / time_per_test.len() as f64;
    let max_time = time_per_test.iter().max().unwrap();
    println!("\nAll tests completed.");
    //println!("Total time: {}μs", total_time);
    println!("Mean time: {}μs", mean_time);
    println!("Max  time: {}μs", max_time);
    if num_fails != 0 {
        eprintln!("ENCOUNTERED FAILURES: {}", num_fails);
    }
}
