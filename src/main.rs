use std::io::stdin;
use std::thread;
use std::time::Duration;

type Board = Vec<Vec<char>>;

const ROWS: usize = 6;
const COLUMNS: usize = 7;
const EMPTY: char = ' ';
const P1: char = 'o';
const P2: char = '△';

// board[ROWS][COLUMNS]
fn main() {
    let mut board = vec![vec![EMPTY; COLUMNS]; ROWS];
    let mut p1turn = true;
    draw_board(&board);
    loop {
        if p1turn {
            println!("Player 1's turn:");
        } else {
            println!("Player 2's turn:");
        }
        let column = get_input();
        match check_column_empty(&board, column) {
            Some(row) => {
                if p1turn {
                    drop_piece(&mut board, row, column, P1);
                    if check_win(&board, P1) {
                        println!("Player 1 wins!");
                        break;
                    }
                } else {
                    drop_piece(&mut board, row, column, P2);
                    if check_win(&board, P2) {
                        println!("Player 2 wins!");
                        break;
                    }
                }
                if check_empty(&board) {
                    println!("DRAW");
                    break;
                }
                p1turn = !p1turn;
            }
            None => {
                println!("Column already full!");
                continue;
            }
        }
    }
}

fn get_input() -> usize {
    let mut buffer = String::new();
    loop {
        println!("Select column 1 -> {COLUMNS}");
        match stdin().read_line(&mut buffer) {
            Err(_) => {
                continue;
            }
            Ok(_) => {
                if let Ok(input) = buffer.trim().parse::<usize>() {
                    if !(input > COLUMNS || input < 1) {
                        return input - 1;
                    }
                    buffer = "".to_string();
                    println!("Not a valid number!");
                    continue;
                } else {
                    println!("Not a valid number!");
                    continue;
                }
            }
        }
    }
}

fn draw_board(board: &Board) {
    clear_terminal();
    println!("{}", "-".repeat(ROWS * 3 - 3));
    print!("|");
    for num in 1..=COLUMNS {
        print!("{}|", num);
    }
    println!("\n{}", "-".repeat(ROWS * 3 - 3));
    for row in board {
        print!("|");
        for column in row {
            print!("{}|", column);
        }
        println!();
    }
}

fn check_column_empty(board: &Board, column: usize) -> Option<usize> {
    for row in (0..ROWS).rev() {
        if board[row][column] == EMPTY {
            return Some(row);
        }
    }
    None
}

fn check_empty(board: &Board) -> bool {
    for col in 0..COLUMNS {
        if check_column_empty(&board, col).is_some() {
            return false;
        }
    }
    true
}

fn drop_piece(board: &mut Board, row: usize, column: usize, chr: char) {
    let ms = Duration::from_millis(20);
    for r in 0..row {
        board[r][column] = chr;
        draw_board(&board);
        thread::sleep(ms);
        board[r][column] = EMPTY;
    }
    board[row][column] = chr;
    draw_board(&board);
}

fn check_win(board: &Board, player: char) -> bool {
    // horizontal check
    for column in 0..COLUMNS - 3 {
        for row in 0..ROWS {
            if board[row][column] == player
                && board[row][column + 1] == player
                && board[row][column + 2] == player
                && board[row][column + 3] == player
            {
                return true;
            }
        }
    }

    // vertical check
    for row in 0..ROWS - 3 {
        for column in 0..COLUMNS {
            if board[row][column] == player
                && board[row + 1][column] == player
                && board[row + 2][column] == player
                && board[row + 3][column] == player
            {
                return true;
            }
        }
    }

    // diagonal check
    for row in 3..ROWS {
        for column in 0..COLUMNS - 3 {
            if board[row][column] == player
                && board[row - 1][column + 1] == player
                && board[row - 2][column + 2] == player
                && board[row - 3][column + 3] == player
            {
                return true;
            }
        }
    }

    // backwards diagnoal check
    for row in 3..ROWS {
        for column in 3..COLUMNS {
            if board[row][column] == player
                && board[row - 1][column - 1] == player
                && board[row - 2][column - 2] == player
                && board[row - 3][column - 3] == player
            {
                return true;
            }
        }
    }
    false
}

#[cfg(any(target_os = "android", target_os = "linux", target_os = "macos"))]
fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[cfg(target_os = "windows")]
fn clear_terminal() {
    print!("{}", "\n".repeat(69));
}
