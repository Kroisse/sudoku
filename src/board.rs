use std::cell::RefCell;

use ndarray::{s, Array2, Axis};
use slint::{Model, ModelNotify, ModelTracker};

use crate::Coord;

pub struct Board {
    cells: RefCell<Array2<u8>>,
    notify: ModelNotify,
}

impl Board {
    pub fn check(&self, coord: Coord, value: u8) -> bool {
        let cells = self.cells.borrow();
        let chunk_row = coord.row as usize / 3 * 3;
        let chunk_col = coord.col as usize / 3 * 3;
        cells
            .slice(s![chunk_row..chunk_row + 3, chunk_col..chunk_col + 3])
            .iter()
            .chain(cells.index_axis(Axis(0), coord.row as usize).iter())
            .chain(cells.index_axis(Axis(1), coord.col as usize).iter())
            .all(|&v| v != value)
    }

    pub fn set(&self, coord: &Coord, value: u8) {
        let mut cells = self.cells.borrow_mut();
        cells[(coord.row as usize, coord.col as usize)] = value;
        self.notify
            .row_changed(coord.row as usize * cells.nrows() + coord.col as usize);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: RefCell::new(Array2::zeros((9, 9))),
            notify: ModelNotify::default(),
        }
    }
}

impl Model for Board {
    type Data = i32;

    fn row_count(&self) -> usize {
        self.cells.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        Some(self.cells.borrow().as_slice()?.get(row)?.clone().into())
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        if let Some(cell) = self
            .cells
            .borrow_mut()
            .as_slice_mut()
            .and_then(|s| s.get_mut(row))
        {
            *cell = data as u8;
            self.notify.row_changed(row);
        }
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }
}
