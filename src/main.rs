mod board;

use std::rc::Rc;

use crate::board::Board;

slint::include_modules!();

fn main() {
    let window = MainWindow::new();

    let board = Rc::new(Board::default());
    board.check(Coord { row: 0, col: 0 }, 1);

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
