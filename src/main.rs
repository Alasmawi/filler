use std::io::{self, BufReader, Write};

use filler::output::format_move;
use filler::parser::{read_player, read_turn};
use filler::strategy::choose_move;

fn main() {
    if let Err(err) = run() {
        eprintln!("filler: {err}");
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let Some(player) = read_player(&mut reader)? else {
        return Ok(());
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    while let Some(turn) = read_turn(&mut reader, player)? {
        let placement = choose_move(&turn.board, &turn.piece);
        stdout.write_all(format_move(placement).as_bytes())?;
        stdout.flush()?;
    }

    Ok(())
}
