use crate::parser::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn own_cells(self) -> [char; 2] {
        match self {
            Player::One => ['a', '@'],
            Player::Two => ['s', '$'],
        }
    }

    pub fn enemy_cells(self) -> [char; 2] {
        match self {
            Player::One => ['s', '$'],
            Player::Two => ['a', '@'],
        }
    }

    pub fn is_own_char(self, cell: char) -> bool {
        self.own_cells().contains(&cell)
    }

    pub fn is_enemy_char(self, cell: char) -> bool {
        self.enemy_cells().contains(&cell)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
    pub player: Player,
}

impl Board {
    pub fn new(
        width: usize,
        height: usize,
        grid: Vec<Vec<char>>,
        player: Player,
    ) -> Result<Self, ParseError> {
        if width == 0 || height == 0 {
            return Err(ParseError::new("board dimensions must be non-zero"));
        }

        if grid.len() != height {
            return Err(ParseError::new(format!(
                "board height mismatch: expected {height}, got {}",
                grid.len()
            )));
        }

        for (idx, row) in grid.iter().enumerate() {
            if row.len() != width {
                return Err(ParseError::new(format!(
                    "board row {idx} width mismatch: expected {width}, got {}",
                    row.len()
                )));
            }
        }

        Ok(Self {
            width,
            height,
            grid,
            player,
        })
    }

    pub fn is_inside(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn is_enemy(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .is_some_and(|cell| self.player.is_enemy_char(cell))
    }

    pub fn is_own(&self, x: usize, y: usize) -> bool {
        self.cell(x, y)
            .is_some_and(|cell| self.player.is_own_char(cell))
    }

    pub fn occupied_count(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&cell| cell != '.')
            .count()
    }

    pub fn empty_neighbor_count(&self, x: usize, y: usize) -> usize {
        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        DIRS.iter()
            .filter_map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.is_inside(nx, ny) {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .filter(|&(nx, ny)| self.cell(nx, ny) == Some('.'))
            .count()
    }

    pub fn enemy_neighbor_count(&self, x: usize, y: usize) -> usize {
        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        DIRS.iter()
            .filter_map(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.is_inside(nx, ny) {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .filter(|&(nx, ny)| self.is_enemy(nx, ny))
            .count()
    }
}
