extern crate crossterm;

use std::borrow::BorrowMut;
use std::io::{stdout, Write};
use std::{env, thread, time};

use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::size,
    Result,
};
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::LifeState::{Alive, Dead};
use clap::{App, Arg};

fn main() -> Result<()> {
    let matches = App::new(env!("APP_NAME"))
        .version(env!("VERSION"))
        .author(env!("AUTHOR_EMAIL"))
        .about("An implementation of the game of life.")
        .arg(
            Arg::with_name("seed")
                .help("(Optional) Provide this to rerun a previous configuration")
                .index(1)
                .required(false),
        )
        .get_matches();

    let args: Vec<String> = env::args().collect();

    let seed: u64 = match matches.value_of("seed") {
        Some(value) => value.parse::<u64>()?,
        None => rand::thread_rng().gen(),
    };

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut cells: Vec<LifeState> = vec![];

    let size = size()?;
    for _ in 0..(size.0 * (size.1)) {
        cells.push(if rng.gen::<bool>() { Alive } else { Dead })
    }

    let mut current_state = Board {
        height: size.1 as usize,
        width: size.0 as usize,
        cells,
    };

    println!("To recreate run:");
    println!("{} {}", args[0], seed);
    // Draw space for the board
    for _ in 0..current_state.height {
        queue!(stdout(), Print("\n"),)?
    }

    loop {
        draw_board(current_state.clone(), rng.borrow_mut())?;
        current_state = next_board_state(current_state.clone());
        let ten_millis = time::Duration::from_millis(500);
        thread::sleep(ten_millis);
    }
}

fn draw_board(board: Board, rng: &mut StdRng) -> Result<()> {
    queue!(
        stdout(),
        cursor::MoveUp(board.height as u16),
        cursor::MoveToColumn(0),
    )?;

    let cells_in_board = board.cells.len();
    for position in 0..cells_in_board {
        let colours = vec![
            Color::DarkGrey,
            Color::DarkRed,
            Color::Green,
            Color::DarkGreen,
            Color::DarkYellow,
            Color::Blue,
            Color::DarkBlue,
            Color::Magenta,
            Color::DarkMagenta,
            Color::Cyan,
            Color::DarkCyan,
            Color::Grey,
        ];

        match board.cells[position] {
            Alive => queue!(
                stdout(),
                SetBackgroundColor(Color::Black),
                SetForegroundColor(*colours.choose(rng).unwrap()),
                Print("█"),
            )?,
            Dead => queue!(
                stdout(),
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::Black),
                Print(" "),
            )?,
        }

        if position + 1 != board.width * board.height && position % board.width == (board.width - 1)
        {
            queue!(stdout(), Print("\n"),)?
        }
    }

    queue!(stdout(), ResetColor,)?;
    stdout().flush()?;
    Ok(())
}

fn next_board_state(board: Board) -> Board {
    let cells_in_board = board.cells.len();
    let mut new_cells: Vec<LifeState> = vec![];

    for position in 0..cells_in_board {
        let neighbours = neighbours(position, board.clone());
        let new_cell = next_cell_state(board.cells[position], neighbours);
        new_cells.push(new_cell)
    }

    Board {
        width: board.width,
        height: board.height,
        cells: new_cells,
    }
}

fn next_cell_state(current: LifeState, neighbours: Neighbours) -> LifeState {
    let neighbour_vec = vec![
        neighbours.0,
        neighbours.1,
        neighbours.2,
        neighbours.3,
        neighbours.4,
        neighbours.5,
        neighbours.6,
        neighbours.7,
    ];

    let alive_count = neighbour_vec
        .iter()
        .filter(|x| LifeState::Alive.eq(x))
        .count();

    if alive_count > 3 {
        return Dead;
    }

    if alive_count < 2 {
        return Dead;
    }

    if alive_count == 4 {
        return Dead;
    }

    if alive_count == 3 {
        return Alive;
    }

    current
}

fn neighbours(position: usize, board: Board) -> Neighbours {
    (
        match get_top_left(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_top(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_top_right(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_left(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_right(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_bottom_left(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_bottom(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
        match get_bottom_right(position, &board) {
            Some(val) => board.cells[val],
            None => Dead,
        },
    )
}

fn get_top_left(position: usize, board: &Board) -> Option<usize> {
    match get_top(position, board) {
        Some(val) => get_left(val, board),
        None => None,
    }
}

fn get_top(position: usize, board: &Board) -> Option<usize> {
    if position < board.width {
        return Some(position + (board.width * (board.height - 1)));
    }

    Some(position - board.width)
}

fn get_top_right(position: usize, board: &Board) -> Option<usize> {
    match get_top(position, board) {
        Some(val) => get_right(val, board),
        None => None,
    }
}

fn get_left(position: usize, board: &Board) -> Option<usize> {
    if position < 1 {
        return Some(position + (board.width - 1));
    }

    if (position + 1) % board.width == 1 {
        return Some(position + (board.width - 1));
    }

    Some(position - 1)
}

fn get_right(position: usize, board: &Board) -> Option<usize> {
    if position + 1 >= board.width * board.height {
        return Some((position + 1) - board.width);
    }

    if (position + 1) % board.width == 0 {
        return Some((position + 1) - board.width);
    }

    Some(position + 1)
}

fn get_bottom_left(position: usize, board: &Board) -> Option<usize> {
    match get_left(position, board) {
        Some(val) => get_bottom(val, board),
        None => None,
    }
}

fn get_bottom(position: usize, board: &Board) -> Option<usize> {
    if position + board.width >= board.width * board.height {
        return Some(position - ((board.height - 1) * board.width));
    }

    Some(position + board.width)
}

fn get_bottom_right(position: usize, board: &Board) -> Option<usize> {
    match get_right(position, board) {
        Some(val) => get_bottom(val, board),
        None => None,
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum LifeState {
    Dead,
    Alive,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Board {
    height: usize,
    width: usize,
    cells: Vec<LifeState>,
}

type Neighbours = (
    LifeState,
    LifeState,
    LifeState,
    LifeState,
    LifeState,
    LifeState,
    LifeState,
    LifeState,
);

#[cfg(test)]
mod tests {
    use crate::LifeState::{Alive, Dead};

    use super::*;

    #[test]
    fn static_states_work() {
        assert_eq!(
            Board {
                height: 4,
                width: 4,
                cells: vec![
                    Dead, Dead, Dead, Dead, Dead, Alive, Alive, Dead, Dead, Alive, Alive, Dead,
                    Dead, Dead, Dead, Dead,
                ],
            },
            next_board_state(Board {
                height: 4,
                width: 4,
                cells: vec![
                    Dead, Dead, Dead, Dead, Dead, Alive, Alive, Dead, Dead, Alive, Alive, Dead,
                    Dead, Dead, Dead, Dead,
                ],
            })
        );
    }

    #[test]
    fn oscillators_work() {
        assert_eq!(
            Board {
                height: 5,
                width: 5,
                cells: vec![
                    Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Alive, Alive,
                    Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                ],
            },
            next_board_state(Board {
                height: 5,
                width: 5,
                cells: vec![
                    Dead, Dead, Dead, Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Alive,
                    Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                ],
            })
        );
    }

    #[test]
    fn any_live_cell_with_fewer_than_two_live_neighbours_dies_as_if_by_underpopulation() {
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Alive, (Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Alive, (Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Dead, (Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead))
        )
    }

    #[test]
    fn any_live_cell_with_two_or_three_live_neighbours_lives_on_to_the_next_generation() {
        assert_eq!(
            LifeState::Alive,
            next_cell_state(Alive, (Alive, Alive, Alive, Dead, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Alive,
            next_cell_state(Alive, (Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Dead, (Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead))
        )
    }

    #[test]
    fn any_live_cell_with_more_than_three_live_neighbours_dies_as_if_by_overpopulation() {
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Alive, (Alive, Alive, Alive, Alive, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Alive, (Alive, Alive, Alive, Alive, Alive, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Dead,
            next_cell_state(Dead, (Alive, Alive, Alive, Alive, Alive, Dead, Dead, Dead))
        );
    }

    #[test]
    fn any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell_as_if_by_reproduction()
    {
        assert_eq!(
            LifeState::Alive,
            next_cell_state(Dead, (Alive, Alive, Alive, Dead, Dead, Dead, Dead, Dead))
        );
        assert_eq!(
            LifeState::Alive,
            next_cell_state(Alive, (Alive, Alive, Alive, Dead, Dead, Dead, Dead, Dead))
        );
    }

    #[test]
    fn when_a_cell_is_off_map_it_is_loops_round() {
        let board = Board {
            cells: vec![
                Alive, Dead, Alive, Dead, Dead, Alive, Dead, Alive, Alive, Dead, Alive, Dead, Dead,
                Alive, Dead, Alive,
            ],
            height: 4,
            width: 4,
        };

        assert_eq!(
            (Alive, Dead, Alive, Dead, Dead, Alive, Dead, Alive),
            neighbours(0, board.clone())
        );

        assert_eq!(
            (Dead, Alive, Dead, Alive, Alive, Dead, Alive, Dead),
            neighbours(4, board.clone())
        );
        assert_eq!(
            (Alive, Dead, Alive, Dead, Dead, Alive, Dead, Alive),
            neighbours(8, board.clone())
        );

        assert_eq!(
            (Dead, Alive, Dead, Alive, Alive, Dead, Alive, Dead),
            neighbours(12, board)
        );
    }

    #[test]
    fn test_get_bottom() {
        let board = Board {
            cells: vec![Dead, Dead, Alive, Dead, Alive, Dead],
            height: 3,
            width: 2,
        };
        assert_eq!(Some(0), get_bottom(4, &board));
        assert_eq!(Some(2), get_bottom(0, &board));
        assert_eq!(Some(4), get_bottom(2, &board));
    }

    #[test]
    fn test_get_top() {
        let board = Board {
            cells: vec![Dead, Alive, Alive],
            height: 3,
            width: 1,
        };
        assert_eq!(Some(0), get_top(1, &board));
        assert_eq!(Some(1), get_top(2, &board));
        assert_eq!(Some(2), get_top(0, &board));
    }

    #[test]
    fn test_get_left() {
        let board = Board {
            cells: vec![Dead, Alive, Alive],
            height: 1,
            width: 3,
        };
        assert_eq!(Some(0), get_left(1, &board));
        assert_eq!(Some(1), get_left(2, &board));
        assert_eq!(Some(2), get_left(0, &board));
    }

    #[test]
    fn test_get_right() {
        let board = Board {
            cells: vec![Dead, Alive, Alive],
            height: 1,
            width: 3,
        };
        assert_eq!(Some(0), get_right(2, &board));
        assert_eq!(Some(1), get_right(0, &board));
        assert_eq!(Some(2), get_right(1, &board));
    }

    #[test]
    fn its_possible_to_get_neighbours_of_a_cell() {
        let board = Board {
            cells: vec![
                Dead, Dead, Dead, Dead, Dead, Alive, Alive, Dead, Dead, Alive, Alive, Dead, Dead,
                Dead, Dead, Dead,
            ],
            height: 4,
            width: 4,
        };

        assert_eq!(
            (
                Dead,  // Top Left
                Dead,  // Top
                Dead,  // Top right
                Dead,  // Left
                Alive, // Right
                Dead,  // Bottom Left
                Alive, // Bottom
                Alive  // Bottom Right
            ),
            neighbours(5, board.clone()),
            "Middle Top Left"
        );
        assert_eq!(
            (
                Dead,  // Top Left
                Dead,  // Top
                Dead,  // Top right
                Alive, // Left
                Dead,  // Right
                Alive, // Bottom Left
                Alive, // Bottom
                Dead   // Bottom Right
            ),
            neighbours(6, board.clone()),
            "Middle Top Right"
        );
        assert_eq!(
            (
                Dead,  // Top Left
                Alive, // Top
                Alive, // Top right
                Dead,  // Left
                Alive, // Right
                Dead,  // Bottom Left
                Dead,  // Bottom
                Dead   // Bottom Right
            ),
            neighbours(9, board.clone()),
            "Middle Bottom Left"
        );
        assert_eq!(
            (
                Alive, // Top Left
                Alive, // Top
                Dead,  // Top right
                Alive, // Left
                Dead,  // Right
                Dead,  // Bottom Left
                Dead,  // Bottom
                Dead   // Bottom Right
            ),
            neighbours(10, board),
            "Middle Bottom Right"
        );
    }
}
