use std::{fs::File, io, io::BufRead, path::Path};
use crate::{Engine, engine::{MoveScore, self}};

#[test]
fn it_read_moves() {
    let moves = read_moves_from_png("cm.pgn").unwrap();
    assert_eq!("d2d4", moves[0]);
}

#[test]
fn it_check() {
    cheat_check(Engine::stockfish().unwrap(), "cm.pgn");
}

#[derive(Debug)]
struct MoveAnalyze {
    best_move: MoveScore,
    move_made: MoveScore,
    is_white: bool,
}
impl MoveAnalyze {
    pub fn print(&self) {
        let same = self.best_move.mv == self.move_made.mv;
        let diff = self.move_made.score - self.best_move.score ;
        let info = if self.is_white {"white"} else {"black"};
        // if same {
        //     println!("{} == engine", info);
        // } else {

        //     println!("{} {} {}", info, same, diff);
        
        // }
        let mut move_score = self.move_made.score;
        let mut best_score = self.best_move.score;
        if !self.is_white {
            move_score *= -1;
            best_score *= -1;
        }
        let e = if same {
            "engine"
        } else {
            ""
        };
        print!("{:>10}  {:>5}  {:>5} (e)  {:>5} (m)", e, diff, best_score, move_score);
        if !self.is_white {
            println!();
        }

    }
}

pub fn cheat_check<T>(mut engine: Engine, file_name: T) 
where
    T: AsRef<Path>,
{
    let moves = read_moves_from_png("cm.pgn").unwrap();
    let mut is_white = true;
    let mut n = 1;
    println!("Check");
    for mv in moves {
        let ma = analyze_and_move(&mut engine, &mv, is_white);
        if is_white {
            print!("{:<3}", n);
            n += 1;
        }
        ma.print();
        is_white = !is_white;
    }
     
}

fn analyze_and_move(engine: &mut Engine, mv: &str, is_white: bool) -> MoveAnalyze {
    let best_move = engine.bestmove();
    let move_made = engine.score(mv);
    engine.make_move(mv);

    MoveAnalyze { best_move, move_made, is_white }
}

pub fn read_moves_from_png<T>(file_name: T) -> io::Result<Vec<String>>
where
    T: AsRef<Path>,
{
    let file = File::open(file_name)?;
    let lines = io::BufReader::new(file).lines();
    let mut moves = vec![];
    for line in lines {
        let line = line?;
        if !line.starts_with("[") && !line.is_empty() {
            let mut m: Vec<String> = line.split(" ")
                .map(String::from)
                .filter(|s| s.len() == 4)
                .collect();
            moves.append(&mut m);
        }
    }
    Ok(moves)
}
