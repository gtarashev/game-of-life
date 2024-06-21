/*          imports             */
/* ============================ */
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{stdout, Stdout};

/*         functions            */
/* ============================ */
pub fn raw_mode() -> Option<RawTerminal<Stdout>> {
    match stdout().into_raw_mode() {
        Ok(raw) => Some(raw),
        Err(e) => None,
    }
}

pub fn new_text_buffer() {
    println!("\x1b[?1049h");
}

pub fn reset_text_buffer() {
    println!("\x1b[?1049l");
}
