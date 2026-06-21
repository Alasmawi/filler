use crate::heatmap::HeatMap;
use crate::placement::{valid_placements, Placement};
use crate::{Board, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GamePhase {
    Early,
    Mid,
    Late,
}

pub fn choose_move(board: &Board, piece: &Piece) -> Placement {
    best_move(board, piece).unwrap_or_else(|| Placement::new(0, 0))
}

pub fn best_move(board: &Board, piece: &Piece) -> Option<Placement> {
    if piece.width > board.width || piece.height > board.height {
        return None;
    }

    let heatmap = HeatMap::from_board(board);
    let phase = game_phase(board);
    let mut best: Option<(Placement, i64)> = None;

    for placement in valid_placements(board, piece) {
        let score = evaluate(board, piece, &heatmap, placement, phase);

        if best
            .as_ref()
            .map_or(true, |(_, best_score)| score < *best_score)
        {
            best = Some((placement, score));
        }
    }

    best.map(|(placement, _)| placement)
}

fn game_phase(board: &Board) -> GamePhase {
    let total = board.width * board.height;
    let occupied = board.occupied_count();
    let ratio = occupied as f64 / total as f64;

    if ratio < 0.15 {
        GamePhase::Early
    } else if ratio < 0.55 {
        GamePhase::Mid
    } else {
        GamePhase::Late
    }
}

fn evaluate(
    board: &Board,
    piece: &Piece,
    heatmap: &HeatMap,
    placement: Placement,
    phase: GamePhase,
) -> i64 {
    let mut heat_score = 0i64;
    let mut center_score = 0i64;
    let mut open_neighbors = 0i64;
    let mut enemy_contacts = 0i64;
    let center_x = (board.width.saturating_sub(1)) as i64;
    let center_y = (board.height.saturating_sub(1)) as i64;

    for &(piece_x, piece_y) in &piece.cells {
        let x = (placement.x + piece_x as isize) as usize;
        let y = (placement.y + piece_y as isize) as usize;

        heat_score += heatmap.score_at(x, y) as i64;
        center_score += ((2 * x as i64 - center_x).abs() + (2 * y as i64 - center_y).abs()) / 2;
        open_neighbors += board.empty_neighbor_count(x, y) as i64;
        enemy_contacts += board.enemy_neighbor_count(x, y) as i64;
    }

    match phase {
        GamePhase::Early => heat_score * 8 + center_score * 3 - open_neighbors,
        GamePhase::Mid => heat_score * 10 + center_score - enemy_contacts * 4,
        GamePhase::Late => heat_score * 4 - open_neighbors * 5 - enemy_contacts * 2,
    }
}
