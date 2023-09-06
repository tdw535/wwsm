pub mod utils {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Cell {
        value: f64,
    }
    #[wasm_bindgen]
    pub struct Universe {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
    }

    #[wasm_bindgen]
    impl Universe {
        pub fn tick(&mut self) {
            let mut next = self.cells.clone();

            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let next_cell_state: f64 = self.determine_next_state(row, col);

                    next[idx].value = next_cell_state;
                }
            }
            self.cells = next;
        }
        pub fn new() -> Universe {
            let width = 64;
            let height = 64;
            let mut cells: Vec<Cell> = Vec::new();
            for _ind in 0..(width * height) {
                cells.push(Cell { value: 0.0 });
            }

            Universe {
                width,
                height,
                cells,
            }
        }

        pub fn render(&self) -> String {
            self.to_string()
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
        pub fn toggle_cell(&mut self, row: u32, column: u32) {
            let idx = self.get_index(row, column);
            self.cells[idx].toggle();
        }
    }

    impl Universe {
        fn get_index(&self, row: u32, column: u32) -> usize {
            (row * self.width + column) as usize
        }
        fn determine_next_state(&self, row: u32, column: u32) -> f64 {
            let idx_cell = self.get_index(row, column);
            let mut cell_state: f64 = self.cells[idx_cell].value;
            let mut cell_count: i32 = 0;
            for delta_row in [self.height - 1, 0, 1].iter().cloned() {
                for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }

                    let neighbor_row = (row + delta_row) % self.height;
                    let neighbor_col = (column + delta_col) % self.width;
                    let idx = self.get_index(neighbor_row, neighbor_col);
                    cell_state += self.cells[idx].value;
                    cell_count += 1;
                }
            }
            let new_state = cell_state / (cell_count as f64);
            new_state.ceil()
        }
    }

    impl Cell {
        fn toggle(&mut self) {
            if self.value < 1.0 {
                self.value = 1.0;
            } else {
                self.value = 0.0;
            }
        }
    }

    use std::fmt;
    impl fmt::Display for Universe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for line in self.cells.as_slice().chunks(self.width as usize) {
                for &cell in line {
                    let symbol = if cell.value < 1.0 { '◻' } else { '◼' };
                    write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }
}
