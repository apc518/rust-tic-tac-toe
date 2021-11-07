use std::io;
use substring::Substring;

fn main() {
    let mut board = [
        ['-'; 3],
        ['-'; 3],
        ['-'; 3]
    ];

    let mut turn: usize = 0; // 0 for O, 1 for X
    const TURN_CHARS: [char; 2] = ['O', 'X'];

    // game loop
    loop {
        println!("Please input your move.");

        let mut user_move = String::new();

        io::stdin()
            .read_line(&mut user_move)
            .expect("Failed to read line");
        
        if user_move == "exit\n" {
            break;
        }

        let move_tuple = parse_move(user_move);

        // if move is out of bounds
        if move_tuple.0 > 2 || move_tuple.1 > 2 {
            println!("Invalid input.");
            continue;
        }
        // if move is to an already occupied square
        if board[move_tuple.0][move_tuple.1] != '-' {
            println!("Invalid move. That square is already occupied.");
            continue;
        }

        board[move_tuple.0][move_tuple.1] = TURN_CHARS[turn % 2];

        println!("Board:");
        for elem in board {
            for s in elem{
                print!("{}", s);
            }
            print!("\n");
        }

        if WIN_CHECK_FUNCS[move_tuple.0][move_tuple.1](board) {
            println!("{} wins!", TURN_CHARS[turn % 2]);
            break;
        }
        else if turn >= 8 {
            println!("Tie.");
            break;
        }

        turn += 1;
    }
}

fn parse_move(s: String) -> (usize, usize) {
    let row: usize = match s.substring(0,1).parse() {
        Ok(num) => num,
        Err(_) => 3
    };

    let col: usize = match s.substring(1,2).parse() {
        Ok(num) => num,
        Err(_) => 3
    };

    (row, col)
}

static WIN_CHECK_FUNCS: [[fn([[char;3];3]) -> bool; 3];3] = [
    [
        |board: [[char;3];3]| { 
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
        },
        |board: [[char;3];3]| {
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
        },
        |board: [[char;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
        }
    ],
    [
        |board: [[char;3];3]| {
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        },
        |board: [[char;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        },
        |board: [[char;3];3]| {
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        }
    ],
    [
        |board: [[char;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
        },
        |board: [[char;3];3]| {
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
        },
        |board: [[char;3];3]| {
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
            ||
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
        }
    ]
];