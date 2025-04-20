[![Docs Status](https://docs.rs/boggle-maker/badge.svg)](https://docs.rs/boggle-maker/)
[![On crates.io](https://img.shields.io/crates/v/boggle-maker.svg)](https://crates.io/crates/boggle-maker)

# `boggle-maker`: Rust tools for Boggle enthusiasts: board generation and word analysis.

This crate implements genetic algorithms for Boggle board generation as part of my ongoing Rust language learning journey. Expect continuous improvements to code quality and data structure design.

# When should I use `boggle-maker`
- **Generating Boggle Boards with Target Scores**: When you need a Boggle board configuration that is likely to yield a specific score, based on a provided dictionary of valid words.
- **Analyzing Boggle Boards**: To thoroughly solve a given Boggle board and obtain detailed information about all valid words present, including their letter paths and individual scores, again using a provided dictionary.

## Examples

To use boggle-maker, add the following to your `Cargo.toml` file:

```toml
[dependencies]
boggle-maker = "0.1.3"
```

### Generating a boggle board with a target score

```rust
let builder = BoggleBuilder::new()
        .with_dictionary_path("word-list.txt")
        .with_target_score(3000)
        .with_length(4)
        .with_width(4);
   
if let Some(board) = builder.build().expect("Failed to load trie from word-list.txt file") {
   println!("This is a generated board by boggle-maker:");
   println!("{:?}", board.hash().to_ascii_uppercase());
   println!("Total score: {}", board.score().unwrap());
}
```

### Solving a vector boggle board 

```rust
let solver = BoggleBoardSolver::new()
    .with_dictionary("word-list.txt");
assert!(solver.is_ok(), "Failed to load trie from word-list.txt file");
let solver = solver.unwrap();
let board = vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S'];
let result = solver.solve_vec(&board, 4, 4).unwrap();
let three_count = result.how_many(3);
println!("There are {three_count} words with score equal to 3");
```

### Solving a boggle board 

```rust
let solver = BoggleBoardSolver::new()
    .with_dictionary("word-list.txt");
assert!(solver.is_ok(), "Failed to load trie from word-list.txt file");
let solver = solver.unwrap();

let board = BoggleBuilder::new()
        .with_dictionary_path("word-list.txt")
        .with_target_score(3000)
        .with_length(4)
        .with_width(4)
        .build()
        .unwrap();
assert!(board.is_some());
let board = board.unwrap();

let result = solver.solve(&board).unwrap();
let three_count = result.how_many(3);
println!("There are {three_count} words with score equal to 3");
```