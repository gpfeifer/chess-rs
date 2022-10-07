use chess::{Engine, cheat_check};

fn main() {
    cheat_check(Engine::stockfish().unwrap(), "cm.pgn");
}
