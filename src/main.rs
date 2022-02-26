mod board;

use std::rc::Rc;

use rand::prelude::*;

use crate::board::Board;

slint::include_modules!();

fn main() {
    let window = MainWindow::new();

    let board = Rc::new(Board::default());

    {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let coord = Coord {
                row: rng.gen_range(0..9),
                col: rng.gen_range(0..9),
            };
            let value = rng.gen_range(1..=9);
            if board.check(&coord, value) {
                board.set(&coord, value);
            }
        }
    }

    window.set_board(Rc::clone(&board).into());

    let w = window.as_weak();
    let b = Rc::clone(&board);
    window.on_cell_clicked(move |num| {
        let window = w.unwrap();
        let coord = window.get_selected_cell();
        b.set(&coord, num as u8);
    });

    window.run();
}
