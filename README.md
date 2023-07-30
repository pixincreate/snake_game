# Snake Xenzia

Execute `cargo doc --open` to get reference to all the methods, functions and libs used.

## Usage

- Clone this repo
- `cargo r`

## Rules

- Hitting walls ends the game
- Self collision ends the game
- Score is printed in console when the game ends
- With every food you eat, you gain 1 point and the snake elongates by 1

## What next?

- Make the code more robust and error safe
- Add bonus food every 5 points

## Tree

```tree
SNAKE_GAME
|   .gitignore
|   Cargo.lock
|   Cargo.toml
|   README.md
|   
+---src
|       food.rs
|       game.rs
|       lib.rs
|       main.rs
|       snake.rs
|       
\---target
```
