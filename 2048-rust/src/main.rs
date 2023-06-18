use rand::random;
use std::{io, ops::AddAssign, thread, time::Duration};

#[derive(Debug)]
enum Move {
    Up,
    Left,
    Down,
    Right,
    Quit,
}

fn index(x: usize, y: usize) -> usize {
    (y * 4) + x
}

fn print_board(field: &mut Vec<usize>) -> Result<(), &'static str> {
    let width = field
        .iter()
        .max()
        .and_then(|x| Some(x.to_string()))
        .and_then(|x| Some(x.len() + 2));

    let spacing = String::from("─").repeat(width.unwrap_or(3));

    println!("┌{0}┬{0}┬{0}┬{0}┐", spacing);

    for ridx in 0..3 {
        print!("│");

        for cidx in 0..4 {
            let idx = index(cidx, ridx);

            match field.get(idx) {
                Some(cell) => {
                    let size = cell.to_string().len();
                    let ws_size = width.unwrap_or(3) - size - 2;
                    let right_ws_size = ws_size / 2;
                    let left_ws_size = ws_size - right_ws_size;

                    let right_ws = String::from(" ").repeat(right_ws_size);
                    let left_ws = String::from(" ").repeat(left_ws_size);

                    if cell == &0 {
                        print!(" {} {} │", left_ws, right_ws);
                    } else {
                        print!(" {}{}{} │", left_ws, cell, right_ws);
                    }
                }
                None => {
                    let err: &'static str = "Index out of range.";
                    return Err(err);
                }
            }
        }
        println!("\n├{0}┼{0}┼{0}┼{0}┤", spacing);
    }

    print!("│");

    for cidx in 0..4 {
        let idx = index(cidx, 3);

        match field.get(idx) {
            Some(cell) => {
                let size = cell.to_string().len();
                let ws_size = width.unwrap_or(3) - size - 2;
                let right_ws_size = ws_size / 2;
                let left_ws_size = ws_size - right_ws_size;

                let right_ws = String::from(" ").repeat(right_ws_size);
                let left_ws = String::from(" ").repeat(left_ws_size);

                if cell == &0 {
                    print!(" {} {} │", left_ws, right_ws);
                } else {
                    print!(" {}{}{} │", left_ws, cell, right_ws);
                }
            }
            None => {
                let err: &'static str = "Index out of range.";
                return Err(err);
            }
        }
    }
    println!("\n└{0}┴{0}┴{0}┴{0}┘", spacing);

    Ok(())
}

fn print_game(
    field: &mut Vec<usize>,
    score: &usize,
    verbose: bool,
    lost: bool,
) -> Result<(), &'static str> {
    match clearscreen::clear() {
        Ok(()) => {}
        Err(_) => return Err("Failed to clear screen."),
    }

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

    print_board(field)?;

    if lost {
        println!("\nGame Over!");
        println!("Final Score: {}", score)
    }

    Ok(())
}

fn do_game_step(
    umove: &Move,
    field: &mut Vec<usize>,
    score: &mut usize,
) -> Result<(), &'static str> {
    match umove {
        &Move::Up => {
            for cidx in 0..4 {
                for ridx in 0..4 {
                    for next in (ridx + 1)..4 {
                        if field[index(cidx, next)] != 0 {
                            if field[index(cidx, ridx)] == 0 {
                                field[index(cidx, ridx)] += field[index(cidx, next)];
                                field[index(cidx, next)] = 0;
                            } else if field[index(cidx, ridx)] == field[index(cidx, next)] {
                                field[index(cidx, ridx)] += field[index(cidx, next)];
                                field[index(cidx, next)] = 0;

                                score.add_assign(field[index(cidx, ridx)]);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        &Move::Left => {
            for ridx in 0..4 {
                for cidx in 0..4 {
                    for next in (cidx + 1)..4 {
                        if field[index(next, ridx)] != 0 {
                            if field[index(cidx, ridx)] == 0 {
                                field[index(cidx, ridx)] += field[index(next, ridx)];
                                field[index(next, ridx)] = 0;
                            } else if field[index(cidx, ridx)] == field[index(next, ridx)] {
                                field[index(cidx, ridx)] += field[index(next, ridx)];
                                field[index(next, ridx)] = 0;

                                score.add_assign(field[index(cidx, ridx)]);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        &Move::Down => {
            for cidx in 0..4 {
                for ridx in (0..4).rev() {
                    for next in (0..ridx).rev() {
                        if field[index(cidx, next)] != 0 {
                            if field[index(cidx, ridx)] == 0 {
                                field[index(cidx, ridx)] += field[index(cidx, next)];
                                field[index(cidx, next)] = 0;
                            } else if field[index(cidx, ridx)] == field[index(cidx, next)] {
                                field[index(cidx, ridx)] += field[index(cidx, next)];
                                field[index(cidx, next)] = 0;

                                score.add_assign(field[index(cidx, ridx)]);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        &Move::Right => {
            for ridx in 0..4 {
                for cidx in (0..4).rev() {
                    for next in (0..cidx).rev() {
                        if field[index(next, ridx)] != 0 {
                            if field[index(cidx, ridx)] == 0 {
                                field[index(cidx, ridx)] += field[index(next, ridx)];
                                field[index(next, ridx)] = 0;
                            } else if field[index(cidx, ridx)] == field[index(next, ridx)] {
                                field[index(cidx, ridx)] += field[index(next, ridx)];
                                field[index(next, ridx)] = 0;

                                score.add_assign(field[index(cidx, ridx)]);
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        _ => return Err("Invalid input."),
    }

    Ok(())
}

fn get_user_move() -> Result<Move, &'static str> {
    let umove;

    'inputloop: loop {
        let mut uinput = String::new();

        match io::stdin().read_line(&mut uinput) {
            Err(_) => {
                let msg: &'static str = "Failed to get user input.";
                return Err(msg);
            }
            Ok(_) => match uinput.chars().nth(0) {
                Some('w') => {
                    umove = Move::Up;
                    break 'inputloop;
                }
                Some('a') => {
                    umove = Move::Left;
                    break 'inputloop;
                }
                Some('s') => {
                    umove = Move::Down;
                    break 'inputloop;
                }
                Some('d') => {
                    umove = Move::Right;
                    break 'inputloop;
                }
                Some('q') => {
                    umove = Move::Quit;
                    break 'inputloop;
                }
                Some(other) => {
                    println!("Invalid input: {}. Valid inputs are w-a-s-d and q.", other);
                    thread::sleep(Duration::from_secs(1));
                }
                _ => {
                    panic!("Unrecognized error!");
                }
            },
        }
    }

    Ok(umove)
}

fn gen_rand_cell(field: &mut Vec<usize>) -> Result<(), &'static str> {
    'randloop: loop {
        let rnd = random::<usize>();
        let idx = index((rnd / 4) % 4, rnd % 4);

        match field.get_mut(idx) {
            None => {
                return Err("Index out of range.");
            }
            Some(cell) => {
                if *cell == 0 {
                    if rnd % 10 == 0 {
                        *cell = 4;
                    } else {
                        *cell = 2;
                    }
                    break 'randloop;
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let mut field = vec![0; 4 * 4];
    let mut score = 0;
    let verbose = false;

    'gameloop: loop {
        match gen_rand_cell(&mut field) {
            Ok(()) => {}
            Err(msg) => panic!("{}", msg),
        }

        for move_type in [Move::Up, Move::Left, Move::Down, Move::Right] {
            let mut test = field.clone();

            match do_game_step(&move_type, &mut test, &mut score) {
                Ok(()) => {}
                Err(msg) => panic!("{}", msg),
            }

            if test != field {
                break;
            }

            match move_type {
                Move::Right => {
                    match print_game(&mut field, &score, verbose, true) {
                        Ok(()) => {}
                        Err(msg) => panic!("{}", msg),
                    }

                    break 'gameloop;
                }
                _ => {}
            }
        }

        match print_game(&mut field, &score, verbose, false) {
            Ok(()) => {}
            Err(msg) => panic!("{}", msg),
        }

        let test = field.clone();

        while test == field {
            match get_user_move() {
                Ok(umove) => match umove {
                    Move::Quit => {
                        println!("\nGame Quit");
                        println!("Final Score: {}", score);

                        match print_game(&mut field, &score, verbose, true) {
                            Ok(()) => {}
                            Err(msg) => panic!("{}", msg),
                        }

                        break 'gameloop;
                    }
                    _ => {
                        match print_game(&mut field, &score, verbose, false) {
                            Ok(()) => {}
                            Err(msg) => panic!("{}", msg),
                        }

                        match do_game_step(&umove, &mut field, &mut score) {
                            Ok(()) => {}
                            Err(msg) => panic!("{}", msg),
                        }
                    }
                },
                Err(msg) => panic!("{}", msg),
            }
        }
    }
}
