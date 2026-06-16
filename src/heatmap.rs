use std::collections::VecDeque;

use crate::Board;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeatMap {
    width: usize,
    height: usize,
    scores: Vec<usize>,
}

impl HeatMap {
    pub fn from_board(board: &Board) -> Self {
        let cell_count = board.width * board.height;
        let mut scores = vec![usize::MAX; cell_count];
        let mut queue = VecDeque::new();

        for y in 0..board.height {
            for x in 0..board.width {
                if board.is_enemy(x, y) {
                    let idx = Self::index(board.width, x, y);
                    scores[idx] = 0;
                    queue.push_back((x, y));
                }
            }
        }

        if queue.is_empty() {
            scores.fill(0);
            return Self {
                width: board.width,
                height: board.height,
                scores,
            };
        }

        while let Some((x, y)) = queue.pop_front() {
            let current = scores[Self::index(board.width, x, y)];
            for (nx, ny) in neighbors(board.width, board.height, x, y) {
                let idx = Self::index(board.width, nx, ny);
                if scores[idx] == usize::MAX {
                    scores[idx] = current + 1;
                    queue.push_back((nx, ny));
                }
            }
        }

        Self {
            width: board.width,
            height: board.height,
            scores,
        }
    }

    pub fn score_at(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            return usize::MAX;
        }

        self.scores[Self::index(self.width, x, y)]
    }

    fn index(width: usize, x: usize, y: usize) -> usize {
        y * width + x
    }
}

fn neighbors(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut result = [(usize::MAX, usize::MAX); 4];
    let mut len = 0usize;

    if x > 0 {
        result[len] = (x - 1, y);
        len += 1;
    }
    if x + 1 < width {
        result[len] = (x + 1, y);
        len += 1;
    }
    if y > 0 {
        result[len] = (x, y - 1);
        len += 1;
    }
    if y + 1 < height {
        result[len] = (x, y + 1);
        len += 1;
    }

    result.into_iter().take(len)
}
