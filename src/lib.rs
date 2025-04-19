//! # boggle-maker
//!
//! A library for generating a boggle board with a target total score.

pub mod builder;
pub mod genetic_boggle_maker;
pub mod simple_genetic_boggle_maker;
pub mod boggle_board;
pub mod boggle_dfs;
pub mod total_boggle_score_calculator;
pub mod boggle_board_solver;

pub use builder::BoggleBuilder;
pub use boggle_board::Board;
pub use boggle_dfs::{WordVisitor,BoggleDfsContext,BoggleDfs};
pub use boggle_board_solver::BoggleBoardSolver;
