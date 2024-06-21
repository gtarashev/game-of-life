/*          modules             */
/* ============================ */
mod grid;
mod output;
mod terminal;

/*          imports             */
/* ============================ */
use grid::{State};
use output::print_grid;
use terminal::{raw_mode, reset_text_buffer, new_text_buffer};
use termion::terminal_size;
use std::process::exit;

/*            main              */
/* ============================ */
fn main() {
    let mut terminal_y = 0;
    let mut terminal_x = 0;
    match terminal_size() {
        Ok(size) => {
            (terminal_x, terminal_y) = size;
        },
        Err(e) => {
            eprintln!("ERROR: {}", e);
            exit(1);
        }
    }

    let mut grid = vec![vec![State::Dead; terminal_x as usize];
                             terminal_y as usize];
    print_grid(&grid);
}
