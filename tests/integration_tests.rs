use std::io::Cursor;

use filler::heatmap::HeatMap;
use filler::output::format_move;
use filler::parser::{parse_board_with_player, parse_piece, parse_player, read_player, read_turn};
use filler::placement::{is_valid, Placement};
use filler::strategy::choose_move;
use filler::{Board, Piece, Player};

fn board(player: Player, rows: &[&str]) -> Board {
    Board::new(
        rows[0].len(),
        rows.len(),
        rows.iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect(),
        player,
    )
    .unwrap()
}

fn piece(rows: &[&str]) -> Piece {
    Piece::from_strings(rows[0].len(), rows.len(), rows).unwrap()
}

#[test]
fn parses_player_symbols() {
    assert_eq!(
        parse_player("$$$ exec p1 : [robots/bender]").unwrap(),
        Player::One
    );
    assert_eq!(
        parse_player("$$$ exec p2 : [robots/bender]").unwrap(),
        Player::Two
    );
}

#[test]
fn parses_board_with_header_and_column_guide() {
    let lines = vec![
        "Anfield 5 3:".to_string(),
        "    01234".to_string(),
        "000 ..@..".to_string(),
        "001 .....".to_string(),
        "002 ..$..".to_string(),
    ];

    let parsed = parse_board_with_player(&lines, Player::One).unwrap();

    assert_eq!(parsed.width, 5);
    assert_eq!(parsed.height, 3);
    assert!(parsed.is_own(2, 0));
    assert!(parsed.is_enemy(2, 2));
}

#[test]
fn parses_piece_cells() {
    let lines = vec![
        "Piece 4 2:".to_string(),
        ".OO.".to_string(),
        "..O.".to_string(),
    ];

    let parsed = parse_piece(&lines).unwrap();

    assert_eq!(parsed.width, 4);
    assert_eq!(parsed.height, 2);
    assert_eq!(parsed.cells, vec![(1, 0), (2, 0), (2, 1)]);
}

#[test]
fn reads_player_and_turn_from_stream() {
    let input = "\
$$$ exec p1 : [robots/bender]
Anfield 5 3:
    01234
000 ..@..
001 .....
002 ..$..
Piece 2 1:
OO
";
    let mut reader = Cursor::new(input);
    let player = read_player(&mut reader).unwrap().unwrap();
    let turn = read_turn(&mut reader, player).unwrap().unwrap();

    assert_eq!(player, Player::One);
    assert_eq!(turn.board.width, 5);
    assert_eq!(turn.piece.cells, vec![(0, 0), (1, 0)]);
}

#[test]
fn accepts_exactly_one_overlap() {
    let board = board(Player::One, &[".....", "..@..", ".....", "..$..", "....."]);
    let piece = piece(&["O"]);

    assert!(is_valid(&board, &piece, 2, 1));
}

#[test]
fn rejects_zero_overlap() {
    let board = board(Player::One, &[".....", "..@..", ".....", "..$..", "....."]);
    let piece = piece(&["O"]);

    assert!(!is_valid(&board, &piece, 0, 0));
}

#[test]
fn rejects_multiple_overlaps() {
    let board = board(Player::One, &[".....", "..@@.", ".....", "..$..", "....."]);
    let piece = piece(&["OO"]);

    assert!(!is_valid(&board, &piece, 2, 1));
}

#[test]
fn rejects_enemy_overlap() {
    let board = board(Player::One, &[".....", "..@..", ".....", "..$..", "....."]);
    let piece = piece(&["O"]);

    assert!(!is_valid(&board, &piece, 2, 3));
}

#[test]
fn rejects_boundary_overflow() {
    let board = board(Player::One, &["@....", ".....", ".....", ".....", "....$"]);
    let piece = piece(&["OO", "OO"]);

    assert!(!is_valid(&board, &piece, -1, 0));
    assert!(!is_valid(&board, &piece, 4, 0));
    assert!(!is_valid(&board, &piece, 0, -1));
    assert!(!is_valid(&board, &piece, 0, 4));
}

#[test]
fn formats_move_as_engine_coordinates() {
    assert_eq!(format_move(Placement::new(7, 2)), "7 2\n");
}

#[test]
fn heatmap_scores_enemy_distance() {
    let board = board(Player::One, &["....$", ".....", "@...."]);
    let heatmap = HeatMap::from_board(&board);

    assert_eq!(heatmap.score_at(4, 0), 0);
    assert_eq!(heatmap.score_at(3, 0), 1);
    assert_eq!(heatmap.score_at(4, 1), 1);
}

#[test]
fn strategy_returns_fallback_when_no_move_exists() {
    let board = board(Player::One, &["@$"]);
    let piece = piece(&["O"]);

    assert_eq!(choose_move(&board, &piece), Placement::new(0, 0));
}

#[test]
fn strategy_returns_valid_move_when_available() {
    let board = board(Player::One, &[".....", "..@..", ".....", "..$..", "....."]);
    let piece = piece(&["OO"]);
    let placement = choose_move(&board, &piece);

    assert!(is_valid(&board, &piece, placement.x, placement.y));
}
