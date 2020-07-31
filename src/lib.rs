mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

use eval::{Expr, to_value};
use std::borrow::Borrow;
use wasm_bindgen::__rt::std::sync::Mutex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl Cell {
    pub fn parse_from_string(str: &String) -> Option<Cell> {
        if str.to_lowercase().contains("alive") {
            return Some(Cell::Alive);
        }
        if str.to_lowercase().contains("dead") {
            return Some(Cell::Dead);
        }
        return None;
    }
}

struct Rule {
    from_state: Cell,
    to_state: Cell,
    with_condition: String
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    rules: Mutex<Vec<Box<Rule>>>
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                //let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                let symbol = if cell == Cell::Dead { '⬜' } else { '⬛' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 32;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 5 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Universe {
            width,
            height,
            cells,
            rules: Mutex::new(vec![])
        }
    }

    pub fn restart(&mut self) {
        let cells = (0..&self.width * &self.height)
            .map(|i| {
                if i % 2 == 0 || i % 5 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        self.cells = cells;
    }

    /// #Returns
    /// index of given point
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// #Returns
    /// live neighbours of given point
    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        // self.height - 1 for universe wrapping
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let index = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[index] as u8;
            }
        }

        count
    }

    /// Function used for passing to js for rendering
    pub fn render(&self) -> String {
        // Implementation of to_string above in impl Display for Universe
        self.to_string()
    }

    /// Generation tick
    /// generates next generation of Game of Life grid
    pub fn tick(&mut self) {
        let mut next_gen = self.cells.clone();

        // generate next_gen
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let neighbours_count = self.live_neighbour_count(row, col);

                // the same by default
                let mut next_cell = cell;

                // iterate through rules, assign next_cell and break
                // when some rule is evaluated to true
                let rules = self.rules.lock().unwrap();
                for i in 0..rules.len() {
                    let rule: &Rule = rules[i].borrow();
                    if cell != rule.from_state {
                        continue;
                    }

                    let expr = Expr::new(rule.with_condition.clone()).value("x", neighbours_count).exec();
                    if expr == Ok(to_value(true)) {
                        next_cell = rule.to_state.clone();
                        break;
                    }
                };

                next_gen[index] = next_cell;
            }
        }

        self.cells = next_gen;
    }

    pub fn add_rule(&mut self, from_state: String, to_state: String, with_condition: String) {
        let mut rules = self.rules.lock().unwrap();
        log(format!("Rule added: {} {} {}", from_state, to_state, with_condition).as_str());
        let from_state_cell = Cell::parse_from_string(&from_state).unwrap();
        let to_state_cell = Cell::parse_from_string(&to_state).unwrap();
        // todo error handling veri important when adding wrong rule the program will crash

        rules.push(Box::new(Rule {
            from_state: from_state_cell.clone(),
            to_state: to_state_cell.clone(),
            with_condition: with_condition.clone()
        }));
    }

    pub fn remove_rule(&mut self, index: usize) {
        self.rules.lock().unwrap().remove(index);
    }
}