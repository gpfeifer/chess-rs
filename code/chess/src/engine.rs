use std::{
    process::{Child, ChildStdin, Command},
    sync::mpsc::Receiver,
};

use command_spawn::spawn_command;
use log::error;
use std::io;
use std::io::Write;

use crate::uci::UciOutput;
#[test]
fn it_calcualtes_scrore() {
    let mut e = Engine::stockfish().unwrap();
    let m = e.bestmove();
    println!("{:?}",m);
    let m = e.bestmove();
    println!("{:?}",m);
    
    let m = e.score("e2e4");
    println!("make move");
    e.make_move("f2f4");
    let m = e.bestmove();
    println!("{:?}",m);

}

#[derive(Debug)]
pub struct MoveScore {
    pub mv: String,
    pub score: i64,
}

pub struct Engine {
    child: Child,
    stdin: ChildStdin,
    out_recv: Receiver<UciOutput>,
    err_recv: Receiver<String>,
    movetime: u32,
    moves: String,
}

impl Engine {
    pub fn stockfish() -> io::Result<Engine> {
        Engine::new("stockfish")
    }

    pub fn new(engine: &str) -> io::Result<Engine> {
        let mut cmd = Command::new(engine);
        let (child, stdin, out_recv, err_recv) = spawn_command::<UciOutput>(&mut cmd)?;
        let movetime = 60000 * 2;
        let moves = " moves ".to_string(); 
        let mut engine = Engine {
            child,
            stdin,
            out_recv,
            err_recv,
            movetime,
            moves,
        
        };

        engine.init_uci();
        Ok(engine)
    }

    pub fn bestmove(&self) -> MoveScore {
        self.command(&format!("go movetime {}", self.movetime)).unwrap();
        let out_vec = self.read_until(&|o| o.is_bestmove());
        self.move_score(&out_vec)
    }

    fn move_score(&self, out_vec: &Vec<UciOutput>) -> MoveScore {
        if out_vec.len() < 2 {
            panic!("PANIC");
        }
        let score = out_vec[out_vec.len()-2].score();
        let mv = out_vec[out_vec.len()-1].bestmove();
        MoveScore { mv, score }
    }

    pub fn score(&self, mv: &str) -> MoveScore {
        self.command(&format!("go movetime {} searchmoves {}", self.movetime, mv)).unwrap();
        let out_vec = self.read_until(&|o| o.is_bestmove());
        self.move_score(&out_vec)
    }

    pub fn is_ready(&self) {
        self.command(&format!("isready")).unwrap();
        let out_vec = self.read_until(&|o| true);
    }

    
    pub fn make_move(&mut self, mv : &str ) {
        self.moves = format!("{} {}", self.moves, mv);
        self.command(&format!("position startpos {}", self.moves)).unwrap();
        self.is_ready();

    }

    fn init_uci(&mut self) -> io::Result<()> {
        self.command("uci")?;
        self.read_until(&|o| o.is_ok());
        Ok(())
    }

    fn command(&self, cmd: &str) -> io::Result<()> {
        writeln!(&self.stdin, "{cmd}")
    }

    fn read_until(&self, predicate: &dyn Fn(&UciOutput) -> bool) -> Vec<UciOutput> {
        let mut search = true;
        let mut result = vec![];
        while search {
            match self.out_recv.recv() {
                Ok(out) => {
                    search = !predicate(&out);
                    result.push(out);
                }
                Err(e) => {
                    error!("read err: {}", e.to_string());
                    search = false;
                }
            }
        }
        result
    }
}
