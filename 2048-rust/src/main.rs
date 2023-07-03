use rand::{seq::SliceRandom, Rng};
use std::{collections::HashMap, io, thread, time::Duration};

type Board = HashMap<(u8, u8), usize>;

#[derive(Debug)]
enum Move {
    Up,
    Left,
    Down,
    Right,
    Quit,
}

fn print_board(board: &Board) {
    let width = board
        .iter()
        .by_ref()
        .map(|(_, v)| v)
        .max()
        .and_then(|x| Some(x.to_string()))
        .and_then(|x| Some(x.len() + 2))
        .unwrap_or(3);

    let spacing = String::from("─").repeat(width);

    println!("┌{0}┬{0}┬{0}┬{0}┐", spacing);

    (0..3).for_each(|y| {
        let row = (0..4).fold(String::from(""), |mut row, x| {
            let tile = board.get(&(x, y)).unwrap().clone();
            let size = tile.to_string().len();

            let ws_size = width - size - 2usize;
            let right_ws_size = ws_size / 2usize;
            let left_ws_size = ws_size - right_ws_size;

            let right_ws = String::from(" ").repeat(right_ws_size);
            let left_ws = String::from(" ").repeat(left_ws_size);

            if tile == 0 {
                row.extend(format!(" {left_ws} {right_ws} │").chars());
            } else {
                row.extend(format!(" {left_ws}{tile}{right_ws} │").chars());
            }

            return row;
        });

        println!("│{}", row);
        println!("├{spacing}┼{spacing}┼{spacing}┼{spacing}┤");
    });

    let row = (0..4).fold(String::from(""), |mut row, x| {
        let tile = board.get(&(x, 3)).unwrap().clone();
        let size = tile.to_string().len();

        let ws_size = width - size - 2usize;
        let right_ws_size = ws_size / 2usize;
        let left_ws_size = ws_size - right_ws_size;

        let right_ws = String::from(" ").repeat(right_ws_size);
        let left_ws = String::from(" ").repeat(left_ws_size);

        if tile == 0 {
            row.extend(format!(" {left_ws} {right_ws} │").chars());
        } else {
            row.extend(format!(" {left_ws}{tile}{right_ws} │").chars());
        }

        return row;
    });

    println!("│{}", row);
    println!("└{spacing}┴{spacing}┴{spacing}┴{spacing}┘");
}

fn show(board: &Board, score: usize, verbose: bool) -> Result<(), String> {
    match clearscreen::clear() {
        Ok(()) => {}
        Err(_) => return Err(String::from("Failed to clear screen.")),
    };

    println!("2048");

    if verbose {
        println!("---------------------");
        println!("\nControls:");
        println!("W - Shift cells up");
        println!("A - Shift cells left");
        println!("S - Shift cells down");
        println!("D - Shift cells right");
        println!("Q - Quit\n");
        println!("Score: {}\n", score);
    } else {
        println!("----");
        println!("Score: {}\n", score);
    }

    print_board(board);
    Ok(())
}

fn input() -> Result<Move, String> {
    loop {
        let mut uinput = String::new();
        match io::stdin().read_line(&mut uinput) {
            Err(_) => return Err(String::from("Failed to get user input.")),
            Ok(_) => {
                return {
                    match uinput.chars().nth(0) {
                        Some('w') => Ok(Move::Up),
                        Some('a') => Ok(Move::Left),
                        Some('s') => Ok(Move::Down),
                        Some('d') => Ok(Move::Right),
                        Some('q') => Ok(Move::Quit),
                        Some(other) => Err(format!(
                            "Invalid input: {}. Valid inputs are w-a-s-d and q.",
                            other
                        )),

                        _ => Err(String::from("Unrecognized error!")),
                    }
                }
            }
        };
    }
}

fn play_move(board: &Board, score: usize, mv: Move) -> (Board, usize) {
    match mv {
        Move::Up => (0..4).fold((board.clone(), score), |(brd, s), x| {
            let tiles = (0..4).map(|y| brd[&(x, y)]).collect::<Vec<_>>();
            let (new_tiles, new_score) = move_and_combine(tiles.as_slice(), s);

            (
                new_tiles
                    .iter()
                    .enumerate()
                    .fold(brd, |mut new_board, (y, &e)| {
                        new_board.insert((x, y as u8), e);
                        return new_board;
                    }),
                new_score,
            )
        }),
        Move::Left => (0..4).fold((board.clone(), score), |(brd, s), y| {
            let tiles = (0..4).map(|x| brd[&(x, y)]).collect::<Vec<_>>();
            let (new_tiles, new_score) = move_and_combine(tiles.as_slice(), s);

            (
                new_tiles
                    .iter()
                    .enumerate()
                    .fold(brd, |mut new_board, (x, &e)| {
                        new_board.insert((x as u8, y), e);
                        return new_board;
                    }),
                new_score,
            )
        }),
        Move::Down => (0..4).fold((board.clone(), score), |(brd, s), x| {
            let tiles = (0..4).rev().map(|y| brd[&(x, y)]).collect::<Vec<_>>();
            let (new_tiles, new_score) = move_and_combine(tiles.as_slice(), s);

            (
                new_tiles
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(brd, |mut new_board, (y, &e)| {
                        new_board.insert((x, y as u8), e);
                        return new_board;
                    }),
                new_score,
            )
        }),
        Move::Right => (0..4).fold((board.clone(), score), |(brd, s), y| {
            let tiles = (0..4).rev().map(|x| brd[&(x, y)]).collect::<Vec<_>>();
            let (new_tiles, new_score) = move_and_combine(tiles.as_slice(), s);

            (
                new_tiles
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(brd, |mut new_board, (x, &e)| {
                        new_board.insert((x as u8, y), e);
                        return new_board;
                    }),
                new_score,
            )
        }),
        _ => panic!(""),
    }
}

fn move_and_combine(tiles: &[usize], score: usize) -> ([usize; 4], usize) {
    let new_tiles = tiles
        .iter()
        .cloned()
        .filter(|&e| e > 0usize)
        .chain([0usize; 4])
        .take(4)
        .collect::<Vec<_>>();

    match *new_tiles.as_slice() {
        [a, b, c, d] if (a == b) && (c == d) => (
            [a * 2usize, c * 2usize, 0usize, 0usize],
            score + (a * 2usize) + (c * 2usize),
        ),
        [a, b, c, d] if (a == b) => ([a * 2usize, c, d, 0usize], score + (a * 2usize)),
        [a, b, c, d] if (b == c) => ([a, b * 2usize, d, 0usize], score + (b * 2usize)),
        [a, b, c, d] if (c == d) => ([a, b, c * 2usize, 0usize], score + (c * 2usize)),
        [a, b, c, d] => ([a, b, c, d], score),
        [] | [_, ..] => ([0usize; 4], score),
    }
}

fn combinable(board: &Board) -> bool {
    (0..4)
        .map(|x| (0..3).map(move |y| board[&(x, y)] == board[&(x, y + 1u8)]))
        .flatten()
        .any(|b| b)
        || (0..4)
            .map(|x| (0..3).map(move |y| board[&(x, y)] == board[&(x + 1u8, y)]))
            .flatten()
            .any(|b| b)
}

fn setup() -> Board {
    let mut board = (0..4)
        .map(|x| (0..4).map(move |y| ((x, y), 0usize)))
        .flatten()
        .collect::<HashMap<(u8, u8), usize>>();

    add_random_tile(&mut board);
    add_random_tile(&mut board);

    return board;
}

fn add_random_tile(board: &mut Board) {
    let mut rng = rand::thread_rng();
    let pos = *blank_spaces(board).choose(&mut rng).unwrap();
    let tile = if rng.gen_range(0..10) == 1 { 4 } else { 2 };
    board.insert(pos, tile);
}

fn blank_spaces(board: &Board) -> Vec<(u8, u8)> {
    board
        .iter()
        .filter_map(|(&k, &v)| if v == 0usize { Some(k) } else { None })
        .collect::<Vec<_>>()
}

fn play(board: Board, score: usize) {
    match show(&board, score, false) {
        Ok(()) => {}
        Err(e) => panic!("{e}"),
    };

    if board.values().any(|&v| v == 0) || combinable(&board) {
        match input() {
            Ok(Move::Quit) => {
                println!("Game Over!");
                println!("Final Score: {score}");
                return ();
            }
            Ok(mv) => {
                let (mut moved, new_score) = play_move(&board, score, mv);

                if moved == board {
                    play(board, new_score);
                } else {
                    add_random_tile(&mut moved);
                    play(moved, new_score);
                }
            }
            Err(e) => {
                println!("{e}");
                thread::sleep(Duration::from_secs(1));
                play(board, score)
            }
        }
    } else {
        println!("Game Over!");
        println!("Final Score: {score}");
    }
}

fn main() {
    let board = setup();
    let score = 0usize;

    play(board, score);
}
