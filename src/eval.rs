/*
    A NOTE ON SCORING:
        A higher score is better for the current player
        the magnitude is the number of stones left it took
        (man idk i'm too tired to write this)
*/

use crate::game;
use crate::transpositions;
use game::{Position, Score};
use transpositions::TTable;

/// Strong finds the evaluation of a Position
/// Weak just determines win/loss/draw
#[allow(dead_code)] // Temporary, until we can pass `-w` to the program
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Strength {
    Strong,
    Weak,
}

pub struct Evaluator {
    t: TTable,
    #[allow(dead_code)]
    strength: Strength,
    gmin: Score,
    gmax: Score,
}

impl Evaluator {
    pub fn new(strength: Strength) -> Self {
        let t = TTable::new();
        let (gmin, gmax) = match strength {
            Strength::Strong => (game::MIN_SCORE, game::MAX_SCORE),
            Strength::Weak => (-1, 1),
        };
        Self {
            t,
            strength,
            gmin,
            gmax,
        }
    }
    pub fn reset(&mut self) {
        self.t.reset();
    }
    pub fn eval(&mut self, g: &Position) -> Score {
        //println!("creating trans");
        //pause();
        // let mut g_scratch = g.clone(); // create a copy once to allow mutation
        // Caching evaluations in a transposition table
        //println!("start eval");
        // Iterative deepening
        solve(g, self.gmin, self.gmax, &mut self.t)
        // Transposition Table
        //negamax_trans(g, self.gmin, self.gmax, &mut self.t)
        // Move ordering with Alpha Beta pruning
        //negamax_ordered(g, self.gmin, self.gmax)
        // Alpha Beta pruning
        //negamax_ab(&mut g_scratch, self.gmin, self.gmax)
        // Min-Max algorithm
        //negamax(&mut g_scratch)
    }
}

#[allow(dead_code)]
pub fn solve(g: &Position, mut min: Score, mut max: Score, t: &mut TTable) -> Score {
    while min < max {
        let mut med = min + (max - min) / 2;
        if med <= 0 && min / 2 < med {
            med = min / 2;
        } else if med >= 0 && max / 2 > med {
            med = max / 2;
        }
        let r = negamax_trans(g, med, med + 1, t);
        if r <= med {
            max = r;
        } else {
            min = r;
        }
    }
    min
}

#[allow(dead_code)]
pub fn negamax_trans(g: &Position, mut alpha: Score, mut beta: Score, t: &mut TTable) -> Score {
    // println!("\n{}", g);
    // println!("Alpha, Beta: {}, {}", alpha, beta);
    // println!("Cached: {}", t.get(g.get_key()));
    // pause();
    //println!("\n{}", g);
    if g.num_moves() >= game::BOARD_SIZE as Score {
        //println!("Draw");
        return 0;
    }
    //println!("Checking for winning moves...");
    for x in 0..game::WIDTH {
        if g.can_play(x) && g.is_winning_move(x) {
            //println!("column {} works!", x+1);
            return (game::MAX_SCORE - g.num_moves()) / 2;
        } //else if g.can_play(x) {
          //    println!("column {} does not work.", x+1);
          //}
    }
    //println!("Checking cache...");
    {
        let cached_eval = t.get(g.get_key());
        //println!("Cached: {}", cached_eval);
        let best_score = if cached_eval == 0 {
            //println!("Miss! :(");
            (game::MAX_SCORE - g.num_moves()) / 2
        } else {
            //print!("h");
            cached_eval + game::MIN_SCORE - 1
        };
        if beta > best_score {
            beta = best_score;
            if alpha >= beta {
                //println!("Return beta");
                return beta;
            }
        }
    }
    //println!("Recursing...");
    for x in [3, 2, 4, 1, 5, 0, 6] {
        if g.can_play(x) {
            //println!("Playing {}; {}, {}", x+1, alpha, beta);
            let mut g2 = g.to_owned();
            g2.play(x);
            let score = -negamax_trans(&g2, -beta, -alpha, t);
            //g.unplay_row(x);
            if score >= beta {
                //println!("Return score");
                return score;
            }
            alpha = alpha.max(score);
            //println!("Played {}, {}, {}", x+1, alpha, beta);
        }
    }
    //println!("Return alpha");
    t.put(g.get_key(), alpha - game::MIN_SCORE + 1);
    alpha
}

// #[allow(dead_code)]
// pub fn negamax_ordered(g: &Position, mut alpha: Score, mut beta: Score) -> Score {
//     //println!("{}", g);
//     if g.num_moves() >= game::BOARD_SIZE as Score {
//         return 0;
//     }
//     //pause();
//     //println!("\n{}", g);
//     for x in 0..game::WIDTH {
//         if g.can_play(x) && g.is_winning_move(x) {
//             //println!("column {} works!", x+1);
//             return (game::MAX_SCORE - g.num_moves()) / 2;
//         } //else if g.can_play(x) {
//         //    println!("column {} does not work.", x+1);
//         //}
//     }
//
//     {
//         let best_score = (game::MAX_SCORE - g.num_moves()) / 2;
//         if beta > best_score {
//             beta = best_score;
//             if alpha >= beta {
//                 return beta;
//             }
//         }
//     }
//
//     for x in [3, 2, 4, 1, 5, 0, 6] {
//         if g.can_play(x) {
//             let mut g2 = g.clone();
//             g2.play(x);
//             let score = -negamax_ordered(&g2, -beta, -alpha);
//             //g.unplay_row(x);
//             if score >= beta {
//                 return score;
//             }
//             alpha = alpha.max(score);
//         }
//     }
//
//     alpha
// }
//
// #[allow(dead_code)]
// pub fn negamax_ab(g: &mut Position, mut alpha: isize, mut beta: isize) -> isize {
//     //println!("{}", g);
//     if g.num_moves() >= game::BOARD_SIZE {
//         return 0;
//     }
//
//     for x in 0..game::WIDTH {
//         if g.can_play(x) && g.is_winning_move(x) {
//             return (game::MAX_SCORE - g.num_moves() as isize) / 2;
//         }
//     }
//
//     {
//         let best_score = (game::MAX_SCORE - g.num_moves() as isize) / 2;
//         if beta > best_score {
//             beta = best_score;
//             if alpha >= beta {
//                 return beta;
//             }
//         }
//     }
//
//     for x in 0..game::WIDTH {
//         if g.can_play(x) {
//             g.play(x);
//             let score = -negamax_ab(g, -beta, -alpha);
//             g.unplay_row(x);
//             if score >= beta {
//                 return score;
//             }
//             alpha = alpha.max(score);
//         }
//     }
//
//     alpha
// }
//
// #[allow(dead_code)]
// pub fn negamax(g: &mut Position) -> isize {
//     //println!("{}", g);
//     if g.num_moves() >= game::BOARD_SIZE {
//         return 0;
//     }
//
//     for x in 0..game::WIDTH {
//         if g.can_play(x) && g.is_winning_move(x) {
//             return (game::BOARD_SIZE + 1 - g.num_moves()) as isize / 2;
//         }
//     }
//
//     let mut best_score = -(game::BOARD_SIZE as isize);
//
//     for x in 0..game::WIDTH {
//         if g.can_play(x) {
//             g.play(x);
//             let score = -negamax(g);
//             g.unplay_row(x);
//             best_score = best_score.max(score);
//         }
//     }
//
//     best_score
// }

#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(dead_code)]
fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    let mut _line = String::new();
    stdin().read_line(&mut _line).unwrap();
}
