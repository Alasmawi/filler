use std::fmt;
use std::io::BufRead;

use crate::{Board, Piece, Player};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub board: Board,
    pub piece: Piece,
}

pub fn parse_player(line: &str) -> Result<Player, ParseError> {
    let normalized = line.to_ascii_lowercase();
    let tokens = normalized.split(|ch: char| !ch.is_ascii_alphanumeric());

    for token in tokens {
        match token {
            "p1" => return Ok(Player::One),
            "p2" => return Ok(Player::Two),
            _ => {}
        }
    }

    Err(ParseError::new(format!(
        "could not parse player from: {line}"
    )))
}

pub fn parse_board(lines: &[String]) -> Result<Board, ParseError> {
    parse_board_with_player(lines, Player::One)
}

pub fn parse_board_with_player(lines: &[String], player: Player) -> Result<Board, ParseError> {
    let header = lines
        .first()
        .ok_or_else(|| ParseError::new("missing board header"))?;
    let (width, height) = parse_dimensions(header, "Anfield")?;
    let mut rows = Vec::with_capacity(height);

    for line in lines.iter().skip(1) {
        if let Some(row) = parse_board_row(line, width)? {
            rows.push(row);
            if rows.len() == height {
                break;
            }
        }
    }

    if rows.len() != height {
        return Err(ParseError::new(format!(
            "missing board rows: expected {height}, got {}",
            rows.len()
        )));
    }

    Board::new(width, height, rows, player)
}

pub fn parse_piece(lines: &[String]) -> Result<Piece, ParseError> {
    let header = lines
        .first()
        .ok_or_else(|| ParseError::new("missing piece header"))?;
    let (width, height) = parse_dimensions(header, "Piece")?;
    let mut rows = Vec::with_capacity(height);

    for line in lines.iter().skip(1).take(height) {
        let row = line.trim().chars().collect::<Vec<char>>();
        if row.len() != width {
            return Err(ParseError::new(format!(
                "piece row width mismatch: expected {width}, got {} in {line:?}",
                row.len()
            )));
        }
        rows.push(row);
    }

    Piece::new(width, height, rows)
}

pub fn read_player<R: BufRead>(reader: &mut R) -> Result<Option<Player>, ParseError> {
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }

        if let Ok(player) = parse_player(line.trim_end()) {
            return Ok(Some(player));
        }
    }
}

pub fn read_turn<R: BufRead>(reader: &mut R, player: Player) -> Result<Option<Turn>, ParseError> {
    let Some(board_header) = read_until_header(reader, "Anfield")? else {
        return Ok(None);
    };

    let (_, board_height) = parse_dimensions(&board_header, "Anfield")?;
    let mut board_lines = Vec::with_capacity(board_height + 2);
    board_lines.push(board_header);
    let mut parsed_board_rows = 0usize;
    let mut line = String::new();

    while parsed_board_rows < board_height {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err(ParseError::new("unexpected EOF while reading board"));
        }

        let owned = line.trim_end_matches(['\r', '\n']).to_string();
        if parse_board_row(&owned, parse_dimensions(&board_lines[0], "Anfield")?.0)?.is_some() {
            parsed_board_rows += 1;
        }
        board_lines.push(owned);
    }

    let board = parse_board_with_player(&board_lines, player)?;
    let Some(piece_header) = read_until_header(reader, "Piece")? else {
        return Err(ParseError::new("missing piece header"));
    };

    let (_, piece_height) = parse_dimensions(&piece_header, "Piece")?;
    let mut piece_lines = Vec::with_capacity(piece_height + 1);
    piece_lines.push(piece_header);

    for _ in 0..piece_height {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err(ParseError::new("unexpected EOF while reading piece"));
        }
        piece_lines.push(line.trim_end_matches(['\r', '\n']).to_string());
    }

    let piece = parse_piece(&piece_lines)?;

    Ok(Some(Turn { board, piece }))
}

fn read_until_header<R: BufRead>(
    reader: &mut R,
    expected_label: &str,
) -> Result<Option<String>, ParseError> {
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }

        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.starts_with(expected_label) {
            return Ok(Some(trimmed.to_string()));
        }
    }
}

fn parse_dimensions(line: &str, expected_label: &str) -> Result<(usize, usize), ParseError> {
    if !line.starts_with(expected_label) {
        return Err(ParseError::new(format!(
            "expected {expected_label} header, got: {line}"
        )));
    }

    let numbers = line
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| ParseError::new(format!("invalid dimension: {err}")))?;

    if numbers.len() < 2 {
        return Err(ParseError::new(format!(
            "expected two dimensions in header: {line}"
        )));
    }

    Ok((numbers[0], numbers[1]))
}

fn parse_board_row(line: &str, width: usize) -> Result<Option<Vec<char>>, ParseError> {
    let trimmed = line.trim_start();
    let Some((row_number, cells)) = trimmed.split_once(char::is_whitespace) else {
        return Ok(None);
    };

    if !row_number.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(None);
    }

    let row = cells.trim().chars().collect::<Vec<char>>();
    if row.len() != width {
        return Err(ParseError::new(format!(
            "board row width mismatch: expected {width}, got {} in {line:?}",
            row.len()
        )));
    }

    if row
        .iter()
        .any(|cell| !matches!(cell, '.' | 'a' | '@' | 's' | '$'))
    {
        return Err(ParseError::new(format!("invalid board row: {line}")));
    }

    Ok(Some(row))
}

#[cfg(test)]
mod tests {
    use super::parse_dimensions;

    #[test]
    fn parses_dimensions() {
        assert_eq!(
            parse_dimensions("Anfield 20 15:", "Anfield").unwrap(),
            (20, 15)
        );
        assert_eq!(parse_dimensions("Piece 4 1:", "Piece").unwrap(), (4, 1));
    }
}
