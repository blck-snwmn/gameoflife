use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> String {
    "test".to_string()
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        *self == Cell::Alive
    }
}

#[wasm_bindgen]
pub struct GameBoard {
    width: u32,
    height: u32,

    // 0: 0~width-1
    // 1: width~2*width-1
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl GameBoard {
    pub fn new(w: u32, h: u32) -> GameBoard {
        let cells: Vec<Cell> = (0..w * h)
            .map(|i| {
                if i % 4 == 0 || i % 7 == 0 || i % 17 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        GameBoard {
            width: w,
            height: h,
            cells: cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_row_col(index: u32, w: u32) -> (u32, u32) {
        let row: u32 = index / w;
        let col: u32 = index % w;
        (row, col)
    }

    fn get_index(row: u32, col: u32, w: u32) -> u32 {
        row * w + col
    }
    fn get_target(index: u32, w: u32, h: u32) -> Vec<(u32, u32)> {
        let (row, col) = GameBoard::get_row_col(index as u32, w);
        let mut xxxx = Vec::with_capacity(8);
        // upper
        if row + 1 < h {
            xxxx.push((row + 1, col));
        }
        // bottom
        if row > 0 {
            xxxx.push((row - 1, col));
        }
        // right
        if col + 1 < w {
            xxxx.push((row, col + 1));
        }
        // left
        if col > 0 {
            xxxx.push((row, col - 1));
        }
        // upper-right
        if row + 1 < h && col + 1 < w {
            xxxx.push((row + 1, col + 1));
        }
        // upper-left
        if row + 1 < h && col > 0 {
            xxxx.push((row + 1, col - 1));
        }
        // bottom-right
        if row > 0 && col + 1 < w {
            xxxx.push((row - 1, col + 1));
        }
        // bottom-left
        if row > 0 && col > 0 {
            xxxx.push((row - 1, col - 1));
        }
        xxxx
    }

    pub fn tick(&mut self) {
        println!("call");
        let next = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, state)| {
                let targets = GameBoard::get_target(index as u32, self.width(), self.height());
                let target_indexies = targets
                    .iter()
                    .map(|(row, col)| GameBoard::get_index(*row, *col, self.width()));
                let target_cells: Vec<&Cell> = target_indexies
                    .map(|index| self.cells.get(index as usize).unwrap_or(&Cell::Dead))
                    .collect();

                let live_num = target_cells.iter().filter(|x| x.is_alive()).count();
                match (*state, live_num) {
                    (Cell::Dead, 3) => Cell::Alive,                     // birth
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, // survival
                    (Cell::Alive, x) if x < 2 => Cell::Dead,            // depopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,            // overcrowding
                    (x, _) => x,
                }
            })
            .collect();
        self.cells = next;
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell;
    use crate::GameBoard;

    #[test]
    fn test_tick() {
        let w: u32 = 5;
        let h: u32 = w;
        let mut cells: Vec<Cell> = (0..w * h).map(|_| Cell::Dead).collect();
        cells[0] = Cell::Alive;
        let mut board = GameBoard {
            width: w,
            height: h,
            cells: cells,
        };
        assert_eq!(board.cells[0], Cell::Alive);
        board.tick();

        assert_eq!(board.cells[0], Cell::Dead);

        board.cells[0] = Cell::Alive;
        board.cells[1] = Cell::Alive;
        board.cells[2] = Cell::Alive;
        board.tick();

        assert_eq!(board.cells[0], Cell::Dead);
        assert_eq!(board.cells[1], Cell::Alive);
        assert_eq!(board.cells[2], Cell::Dead);
    }
    #[test]
    fn test_get_col_row() {
        assert_eq!(GameBoard::get_row_col(0, 10), (0, 0));
        assert_eq!(GameBoard::get_row_col(1, 10), (0, 1));
        assert_eq!(GameBoard::get_row_col(10, 10), (1, 0));
        assert_eq!(GameBoard::get_row_col(21, 10), (2, 1));
    }
    #[test]
    fn test_get_index() {
        assert_eq!(GameBoard::get_index(0, 0, 10), 0);
        assert_eq!(GameBoard::get_index(0, 1, 10), 1);
        assert_eq!(GameBoard::get_index(1, 0, 10), 10);
        assert_eq!(GameBoard::get_index(2, 1, 10), 21);
    }

    #[test]
    fn test_get_target() {
        assert_eq!(
            GameBoard::get_target(0, 3, 3).sort(),
            vec![(0, 1), (1, 0), (1, 1)].sort()
        );
        assert_eq!(
            GameBoard::get_target(2, 3, 3).sort(),
            vec![(0, 1), (1, 1), (1, 2)].sort()
        );
        assert_eq!(
            GameBoard::get_target(4, 3, 3).sort(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
            .sort()
        );
        assert_eq!(
            GameBoard::get_target(6, 3, 3).sort(),
            vec![(1, 0), (1, 1), (2, 1)].sort()
        );
        assert_eq!(
            GameBoard::get_target(8, 3, 3).sort(),
            vec![(1, 1), (1, 2), (2, 1)].sort()
        );
        assert_eq!(
            GameBoard::get_target(3, 3, 3).sort(),
            vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)].sort()
        );
    }
}
