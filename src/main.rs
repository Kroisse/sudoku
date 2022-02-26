mod board;

use std::rc::Rc;

use crate::board::Board;

slint::include_modules!();

fn main() {
    let window = MainWindow::new();

    let board = Rc::new(Board::default());
    board.check(Coord { row: 0, col: 0 }, 1);

    window.set_board(board.into());

    window.on_cell_clicked(|coord| {
        println!("{:?}", coord);
    });

    window.run();
}
