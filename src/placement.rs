use crate::{Board, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub x: isize,
    pub y: isize,
}

impl Placement {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub fn is_valid(board: &Board, piece: &Piece, x: isize, y: isize) -> bool {
    if x < 0
        || y < 0
        || x + piece.width as isize > board.width as isize
        || y + piece.height as isize > board.height as isize
    {
        return false;
    }

    let mut overlap = 0usize;

    for &(piece_x, piece_y) in &piece.cells {
        let board_x = x + piece_x as isize;
        let board_y = y + piece_y as isize;

        if !board.is_inside(board_x, board_y) {
            return false;
        }

        let board_x = board_x as usize;
        let board_y = board_y as usize;

        if board.is_enemy(board_x, board_y) {
            return false;
        }

        if board.is_own(board_x, board_y) {
            overlap += 1;
            if overlap > 1 {
                return false;
            }
        }
    }

    overlap == 1
}

pub fn valid_placements<'a>(
    board: &'a Board,
    piece: &'a Piece,
) -> impl Iterator<Item = Placement> + 'a {
    let max_x = board.width.saturating_sub(piece.width);
    let max_y = board.height.saturating_sub(piece.height);
    let piece_fits = piece.width <= board.width && piece.height <= board.height;

    (0..=max_y)
        .flat_map(move |y| (0..=max_x).map(move |x| Placement::new(x as isize, y as isize)))
        .filter(move |placement| piece_fits && is_valid(board, piece, placement.x, placement.y))
}
