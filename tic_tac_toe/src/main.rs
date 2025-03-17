use std::io;

const EMPTY: char = ' ';

fn print_board(board: &[[char; 3]; 3]) {
    println!("_____________");
    for row in board {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        println!("--------------");
    }
}

fn check_winner(board: &[[char; 3]; 3], player: char) -> bool {
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player)
            || (board[0][i] == player && board[1][i] == player && board[2][i] == player)
        {
            return true;
        }
    }
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
    {
        return true;
    }
    false
}

fn is_draw(board: &[[char; 3]; 3]) -> bool {
    board
        .iter()
        .all(|row| row.iter().all(|&cell| cell != EMPTY))
}

fn main() {
    let mut board = [[EMPTY; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_board(&board);
        println!(
            "Player {}'s turn. Enter row and column (0-2):",
            current_player
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let inputs: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        if inputs.len() != 2 || inputs[0] > 2 || inputs[1] > 2 {
            println!("Invalid input. Please enter two numbers between 0 and 2.");
            continue;
        }
        let (row, col) = (inputs[0], inputs[1]);

        if board[row][col] != EMPTY {
            println!("Cell already occupied. Choose another cell.");
            continue;
        }

        board[row][col] = current_player;

        if check_winner(&board, current_player) {
            print_board(&board);
            println!("Player {} wins!", current_player);
            break;
        }

        if is_draw(&board) {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
