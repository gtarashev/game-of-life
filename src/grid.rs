/*          structs             */
/* ============================ */
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

/*          functions           */
/* ============================ */
/* returns the number of neighbours of a cell */
fn get_n(grid: &Vec<Vec<State>>, x: isize, y: isize) -> usize {
    let mut x_com = [x, 0, 0];
    let mut y_com = [y, 0, 0];
    x_com[2] = x + 1;
    x_com[1] = x - 1;
    y_com[2] = y + 1;
    y_com[1] = y - 1;

    if x == 0 {
        x_com[1] = grid[0].len() as isize;
    } else if x == grid[0].len() as isize{
        x_com[2] = 0;
    }

    if y == 0 {
        y_com[1] = grid[0].len() as isize;
    } else if y == grid[0].len() as isize{
        y_com[2] = 0;
    }

    /* go through all combos and add up neighbours */
    let mut n = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == 0 && j == 0 {
                continue; /* dont want to check the cell itself */
            }

            match grid[y_com[i] as usize][x_com[j] as usize] {
                State::Alive => n += 1,
                State::Dead => (),
            }
        }
    }

    return n;
}

pub fn update_state(grid: &mut Vec<Vec<State>>) {
    let mut temp_grid = vec![vec![State::Dead; grid[0].len()]; grid.len()];
    let mut n = 0;
    for y in 0..grid.len() as isize{
        for x in 0..grid[0].len() as isize{
            n = get_n(&grid, x, y);
            if n == 3 {
                temp_grid[y as usize][x as usize] = State::Alive;
                continue;
            } else if n == 2 && temp_grid[y as usize][x as usize] == State::Alive {
                continue;
            }

            temp_grid[y as usize][x as usize] = State::Dead;
        }
    }
    *grid = temp_grid;
}

