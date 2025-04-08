//! # boggle-maker
//!
//! A library for generating a boggle board with a target total score.

pub mod builder;
pub mod genetic_boggle_maker;
pub mod simple_genetic_boggle_maker;
pub mod boggle_board;

pub use builder::BoggleBuilder;
pub use boggle_board::Board;
