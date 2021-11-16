use std::io;
use substring::Substring;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Square {
    O,
    X,
    Empty
}

fn char_of_square(sq: Square) -> char {
    match sq {
        Square::O => 'O',
        Square::X => 'X',
        Square::Empty => ' ',
    }
}

fn main() {
    let mut board = [
        [Square::Empty; 3],
        [Square::Empty; 3],
        [Square::Empty; 3]
    ];

    let mut turn: usize = 0; // 0 for O, 1 for X
    const TURN_CHARS: [Square; 2] = [Square::O, Square::X];
    
    println!("RUST TIC-TAC-TOE BY ANDY CHAMBERLAIN\nEnter moves in the form `rc`\nwhere r is the row and c is the column\nMoving to the top right corner would look like `02`");

    print_board(board);

    // game loop
    loop {
        println!("{}'s turn. Please enter a move.", char_of_square(TURN_CHARS[turn % 2]));

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
        if board[move_tuple.0][move_tuple.1] != Square::Empty {
            println!("Invalid move. That square is already occupied.");
            continue;
        }

        board[move_tuple.0][move_tuple.1] = TURN_CHARS[turn % 2];

        print_board(board);

        if WIN_CHECK_FUNCS[move_tuple.0][move_tuple.1](board) {
            println!("{} wins!", char_of_square(TURN_CHARS[turn % 2]));
            break;
        }
        else if turn >= 8 {
            println!("Tie.");
            break;
        }

        turn += 1;
    }
}

fn print_board(board: [[Square; 3]; 3]){
    print!("\n");
    for i in 0..3 {
        for k in 0..3 {
            print!(" {} ", char_of_square(board[i][k]));
            if k < 2 {
                print!("|");
            }
        }
        print!("\n");
        if i < 2{
            println!("---|---|---");
        }
    }
    print!("\n");
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

static WIN_CHECK_FUNCS: [[fn([[Square;3];3]) -> bool; 3];3] = [
    [
        |board: [[Square;3];3]| { 
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
        },
        |board: [[Square;3];3]| {
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
        },
        |board: [[Square;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[0][1] && board[0][1] == board[0][2] // top row
            ||
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
        }
    ],
    [
        |board: [[Square;3];3]| {
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        },
        |board: [[Square;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        },
        |board: [[Square;3];3]| {
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
            ||
            board[1][0] == board[1][1] && board[1][1] == board[1][2] // middle row
        }
    ],
    [
        |board: [[Square;3];3]| {
            board[0][2] == board[1][1] && board[1][1] == board[2][0] // forward slash diagonal (/)
            ||
            board[0][0] == board[1][0] && board[1][0] == board[2][0] // left column
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
        },
        |board: [[Square;3];3]| {
            board[0][1] == board[1][1] && board[1][1] == board[2][1] // middle column
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
        },
        |board: [[Square;3];3]| {
            board[0][0] == board[1][1] && board[1][1] == board[2][2] // backslash diagonal (\)
            ||
            board[2][0] == board[2][1] && board[2][1] == board[2][2] // bottom row
            ||
            board[0][2] == board[1][2] && board[1][2] == board[2][2] // right column
        }
    ]
];