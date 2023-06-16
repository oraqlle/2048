use std::{thread, time::Duration};

enum Move {
    Up,
    Down,
    Left,
    Right,
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

    let spacing = String::from("-").repeat(width.unwrap_or(3));
    let divider = format!("+{0}+{0}+{0}+{0}+", spacing);

    println!("{}", divider);

    for ridx in 0..4 {
        print!("|");

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

                    print!(" {}{}{} |", left_ws, cell, right_ws);
                }
                None => {
                    panic!("Index out of range: {}", idx);
                }
            }
        }

        println!("\n{}", divider);
    }

    Ok(())
}

fn main() {
    let mut field = vec![0; 4 * 4];

    match print_board(&mut field) {
        Ok(()) => {}
        Err(msg) => {
            panic!("{}", msg);
        }
    }

    thread::sleep(Duration::from_secs(5));

    match clearscreen::clear() {
        Ok(()) => {}
        Err(msg) => {
            panic!("{}", msg);
        }
    }

    field[2] = 2;

    match print_board(&mut field) {
        Ok(()) => {}
        Err(msg) => {
            panic!("{}", msg);
        }
    }

    thread::sleep(Duration::from_secs(5));

    match clearscreen::clear() {
        Ok(()) => {}
        Err(msg) => {
            panic!("{}", msg);
        }
    }

    field[7] = 2222;

    match print_board(&mut field) {
        Ok(()) => {}
        Err(msg) => {
            panic!("{}", msg);
        }
    }
}
