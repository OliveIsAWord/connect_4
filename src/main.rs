mod eval;
mod game;
mod play;
mod transpositions;

use eval::{Evaluator, Strength};
use game::{Position, Score};

//use std::error::Error;
use std::env;
use std::fs;
//use std::process;
use std::time::SystemTime;

//const TEST_FILEPATH: &str = "tests/Test_L2_R1";

const STRENGTH: Strength = Strength::Weak;

#[derive(Debug)]
struct Test {
    moves: String,
    eval: Score,
}

fn get_tests(filepath: &str) -> Vec<Test> {
    let raw = fs::read_to_string(filepath).unwrap();
    let lines = raw.lines();
    let mut tests = Vec::new();
    for line in lines {
        let idx = line.find(' ').unwrap();
        let moves = line[..idx].to_string();
        let eval = line[idx + 1..].parse().unwrap();
        tests.push(Test { moves, eval })
    }
    tests
}

//#[allow(unreachable_code)]
fn main() {
    //play::main();
    //panic!("uwu\nuwu\nuwu\nuwu\nuwu\nuwu\nuwu\nuwu\n");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected arguments. The only argument is the filepath of the test.");
    }
    let fp = &args[1];
    println!("Getting tests from {}...", fp);
    let tests = get_tests(fp);
    let mut total_time = 0; // total time per test in microseconds|
    println!("Initializing transposition table...");
    let mut my_eval = Evaluator::new(STRENGTH);
    let mut max_time = 0;
    println!("Running {} tests...", tests.len());
    //let mut i = 0;
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
                total_time += test_time;
                max_time = max_time.max(test_time);
            }
            Err(e) => println!("Timing Error: {}", e),
        }

        let failed = match STRENGTH {
            Strength::Strong => score != test.eval,
            Strength::Weak => score != test.eval.signum(),
        };
        //println!("{} {}", score, test.eval.signum());
        if failed {
            eprintln!("Test failed: {:?}", test);
            eprintln!("{}", my_game);
            println!("Score:    {}", score);
            println!("Expected: {}", test.eval);
        }

        // let ekey = my_game.get_key();
        // let e = Entry::from_pos(ekey, score);
        // let epos = Position::from_key(e.get_key());
        //println!("{:064b}", ekey);
        //println!("{}", e.bit_string());
        //println!("{}", epos);
        //assert!(my_game == epos);
        //i += 1;
        // if i >= 2 {
        //     break;
        // }
    }
    let mean_time = total_time as f64 / tests.len() as f64;
    println!("All tests completed.");
    println!("Mean time: {}μs", mean_time);
    println!("Max  time: {}μs", max_time);
}
