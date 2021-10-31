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

        println!("Your move was {}", user_move);

        let move_tuple = parse_move(user_move);

        if move_tuple.0 > 2 || move_tuple.1 > 2 {
            println!("Invalid input.");
            continue;
        }

        println!("Parsed: {}, {}", move_tuple.0, move_tuple.1);

        board[move_tuple.0][move_tuple.1] = TURN_CHARS[turn];

        println!("Board:");
        for elem in board {
            for s in elem{
                print!("{}", s);
            }
            print!("\n");
        }

        turn = (turn + 1) % 2;
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