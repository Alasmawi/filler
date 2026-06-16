use crate::Placement;

pub fn format_move(placement: Placement) -> String {
    format!("{} {}\n", placement.x, placement.y)
}
