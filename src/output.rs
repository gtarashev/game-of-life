/*          imports             */
/* ============================ */
use crate::grid::State;

/*         functions            */
/* ============================ */
pub fn print_grid(grid: &Vec<Vec<State>>) {
    for i in grid {
        for j in i {
            match j {
                State::Dead => print!("{}", '.'),
                State::Alive => print!("{}", '#'),
            }
        }
        print!("\r\n");
    }
}
