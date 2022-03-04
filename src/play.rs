use crate::game;
use game::Position;
use std::io::{stdin, stdout, Write};

#[allow(dead_code)]
pub fn main() {
    let mut pos = Position::new();
    let mut last_played = 0;
    loop {
        println!("{}", pos);
        let x = get_input(&pos);
        if x == -1 {
            pos.unplay_row(last_played);
        } else if pos.is_winning_move(x as usize) {
            println!("Win!");
        } else {
            pos.play(x as usize);
            last_played = x as usize;
        }
    }
}

fn get_input(pos: &Position) -> isize {
    loop {
        let mut stdout = stdout();
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();
        if input.eq("undo") || input.eq("u") {
            return -1;
        }
        match input.trim().parse::<isize>() {
            Ok(n) => {
                //println!("{}", pos.can_play(n - 1));
                if n < 1 || n > game::WIDTH as isize {
                    println!("Error: Value must be between 1 and {}.", game::WIDTH);
                } else if !pos.can_play((n - 1) as usize) {
                    println!("Error: Can't play there.");
                } else {
                    return n - 1;
                }
            }
            Err(_) => {
                println!("Error: Value must be a whole number.");
            }
        }
    }
}
