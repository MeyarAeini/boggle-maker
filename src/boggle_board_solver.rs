use std::collections::HashSet;
use word_trie::trie::Trie;
use word_trie::TrieBuilder;
use crate::boggle_dfs::{WordVisitor,BoggleDfsContext,BoggleDfs,get_word_score};
use crate::boggle_board::Board;

/// Boggle board result after dfs search
pub struct BoggleBoardResult {
    words: HashSet<String>,
    path_tracks: Vec<Vec<u16>>,
    counts: Vec<u32>,
}

impl BoggleBoardResult {
    /// Initiate a new instance of BoggleBoardResult.
    pub fn new() -> Self {
        Self{
            words: HashSet::new(),
            path_tracks: Vec::new(),
            counts : vec![0;12], 
        }
    }

    /// gets the result for query of how many words exists in the board with specefic score.
    pub fn how_many(&self, score: usize) -> u32 {        
        if score > 11 { 0 } else { self.counts[score] }        
    }

    /// gets the board's all words hash set
    pub fn words(&self) -> &HashSet<String> {
        &self.words
    }

    /// gets a vector of all valid paths in the boggle board.
    pub fn path_tracks(&self) -> &Vec<Vec<u16>> {
        &self.path_tracks
    }

    pub(crate) fn add_word(&mut self, word: String){
        //update the score map
        let score = get_word_score(&word);
        self.inc_score(score);

        self.words.insert(word);        
    }

    pub(crate) fn add_path(&mut self, path: Vec<u16>){
        self.path_tracks.push(path);
    }

    fn inc_score(&mut self, score: u32){
        Self::score_guard(score);
        self.counts[score as usize] += 1;
    }

    fn score_guard(score: u32){
        if score > 12 {
            panic!("the functionality has not been implemented for score bigger than 11");
        }
    }
}

struct BoggleBoardSolverVisitor(BoggleBoardResult);

impl WordVisitor for BoggleBoardSolverVisitor {
    fn visit(&mut self, word: &str, path: &Vec<u16>){

        //add the path to path_tracks
        self.0.add_path(path.to_vec());

        if self.0.words().contains(word) {
            return;
        }

        //add the word to visited list
        self.0.add_word(word.to_string());
    }
}

/// the boggle board solver struct
#[derive(Default)]
pub struct BoggleBoardSolver (Option<Trie>); 

impl BoggleBoardSolver {
    /// gets new instance of BoggleBoardSolver
    pub fn new() -> Self {
        Self::default()
    }

    ///sets the trie dictionary text file path
    pub fn with_dictionary<P: Into<String>>(mut self, path: P) -> Result<Self, std::io::Error> {        
        self.0 = Some(TrieBuilder::new()
        .from_file(path.into())
        .expect("Failed to load trie from file"));
        
        Ok(self)
    } 

    /// solve a vector representing the boggle board
    pub fn solve_vec(&self, board: &Vec<char>, width: usize, length: usize) -> Option<BoggleBoardResult> {
        match &self.0 {
            Some(trie) => {
                let context = BoggleDfsContext::new(&trie, width, length);

                if board.len() != context.count() {
                    panic!("The board size must be fit to the length:{0} and width:{1}", length, width);
                }

                let mut visitor = BoggleBoardSolverVisitor(BoggleBoardResult::new());
                BoggleDfs::new(&context, board).with_visitor(&mut visitor).search();

                Some(visitor.0)
            },
            None => None
        }
    }

    /// solve a boggle board
    pub fn solve(&self, board: &Board) -> Option<BoggleBoardResult> {
        self.solve_vec(board.value(), board.width(), board.length())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::builder::BoggleBuilder;

    #[test]
    fn there_should_be_atleast_10_words_with_score_3(){
        let solver = BoggleBoardSolver::new()
            .with_dictionary("words.txt");

        assert!(solver.is_ok(), "Failed to load trie from file");

        let solver = solver.unwrap();

        let board = vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S'];

        let result = solver.solve_vec(&board, 4, 4);
        assert!(result.is_some(), "the result does not have value");

        let result = result.unwrap();
        let three_count = result.how_many(3);
        assert!(three_count>10);
        println!("There are {three_count} words with score equal to 3");
    }

    #[test]
    fn there_should_be_no_words_with_score_4(){
        let solver = BoggleBoardSolver::new()
            .with_dictionary("words.txt");

        assert!(solver.is_ok(), "Failed to load trie from file");

        let solver = solver.unwrap();

        let board = vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S'];

        let result = solver.solve_vec(&board, 4, 4);
        assert!(result.is_some(), "the result does not have value");

        let result = result.unwrap();

        let four_count = result.how_many(4);
        assert_eq!(four_count,0);
        println!("There are {four_count} words with score equal to 4");
    }

    #[test]
    fn there_should_be_some_words_with_3_scores_for_a_board_generated(){
        let solver = BoggleBoardSolver::new()
                .with_dictionary("words.txt");
        assert!(solver.is_ok(), "Failed to load trie from words.txt file");
        let solver = solver.unwrap();

        let board = BoggleBuilder::new()
                .with_dictionary_path("words.txt")
                .with_target_score(3000)
                .with_length(4)
                .with_width(4)
                .build()
                .unwrap();
        assert!(board.is_some());
        let board = board.unwrap();
        
        let result = solver.solve(&board).unwrap();
        let three_count = result.how_many(3);
        assert!(three_count > 0);
        println!("There are {three_count} words with score equal to 3");
    }
}