use crate::parser::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<(usize, usize)>,
}

impl Piece {
    pub fn new(width: usize, height: usize, rows: Vec<Vec<char>>) -> Result<Self, ParseError> {
        if width == 0 || height == 0 {
            return Err(ParseError::new("piece dimensions must be non-zero"));
        }

        if rows.len() != height {
            return Err(ParseError::new(format!(
                "piece height mismatch: expected {height}, got {}",
                rows.len()
            )));
        }

        let mut cells = Vec::new();

        for (y, row) in rows.iter().enumerate() {
            if row.len() != width {
                return Err(ParseError::new(format!(
                    "piece row {y} width mismatch: expected {width}, got {}",
                    row.len()
                )));
            }

            for (x, &cell) in row.iter().enumerate() {
                if cell != '.' {
                    cells.push((x, y));
                }
            }
        }

        if cells.is_empty() {
            return Err(ParseError::new(
                "piece must contain at least one active cell",
            ));
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }

    pub fn from_strings(width: usize, height: usize, rows: &[&str]) -> Result<Self, ParseError> {
        let rows = rows
            .iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        Self::new(width, height, rows)
    }
}
