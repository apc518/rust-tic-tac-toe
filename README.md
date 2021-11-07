# Rust Tic-Tac-Toe

I'm learning Rust, and one of the things I do in any new language is code up a simple game of terminal tic-tac-toe, with input validation and gamestate checking.

A move looks like `rc` where r is the row and c is the column, 0-indexed of course ;)

So moving to the center would be `11`, moving to the bottom left corner would be `20`, etc

```
$ cargo run 
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