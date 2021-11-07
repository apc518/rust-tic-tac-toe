# Rust Tic-Tac-Toe

I'm learning Rust, and one of the things I do in any new language is code up a simple game of terminal tic-tac-toe, with input validation and gamestate checking.

## Usage
You must have [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.
```
git clone https://github.com/apc518/rust-tic-tac-toe.git
cd rust-tic-tac-toe
cargo run
```

## Interface
A move looks like `rc` where r is the row and c is the column, 0-indexed of course ;)

So moving to the center would be `11`, moving to the bottom left corner would be `20`, etc

## Example
```
O's turn. Please enter a move.
02

   |   | O 
---|---|---
   |   |   
---|---|---
   |   |   

X's turn. Please enter a move.
11

   |   | O 
---|---|---
   | X |   
---|---|---
   |   |   

...

O's turn. Please enter a move.
12

 X |   | O 
---|---|---
   | X | O 
---|---|---
 O | X | O 

O wins!
```

## Unusual win state checking

In coding terminal tic-tac-toe for every new language I learn, I've come across many ways of checking for a win state in a tic-tac-toe game. The method employed in this project is quite different from any I've used before: it checks every move to see if that move specifically has caused a three-in-a-row, but does so without having to check the whole board.

It is made super efficient by omitting if statements, opting instead to use a 2d array of functions (courtesy of Rust's lambdas), each of which returns whether there is a three in a row that intersects the respective square on the board. So, a move can be mapped directly to a function that does a minimal amount of work to determine if that move resulted in a win.

There is also the added convenience that tic-tac-toe shares with tennis and many other games: the player who moves last ≡ the winner, so you don't actually have to check which player has the three in a row. If a move results in a three-in-a-row, then the player that made that move wins.