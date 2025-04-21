use std::collections::{HashSet, HashMap};
use word_trie::trie::Trie;
use word_trie::TrieBuilder;
use crate::boggle_dfs::{WordVisitor,BoggleDfsContext,BoggleDfs,get_word_score};
use crate::boggle_board::Board;

/// Boggle board result after dfs search
pub struct BoggleBoardResult {
    words: HashSet<String>,
    path_tracks: Vec<Vec<u16>>,
    counts: Vec<u32>,
    length_map: HashMap<usize, usize>,
}

impl BoggleBoardResult {
    /// Initiate a new instance of BoggleBoardResult.
    pub fn new() -> Self {
        Self{
            words: HashSet::new(),
            path_tracks: Vec::new(),
            counts: vec![0;12], 
            length_map: HashMap::new(),
        }
    }

    /// gets the result for query of how many words exists in the board with specefic score.
    pub fn how_many(&self, score: usize) -> u32 {        
        if score > 11 { 0 } else { self.counts[score] }        
    }

    /// gets a refrence to all score counts vector
    pub fn score_counts(&self) -> &Vec<u32> {
        &self.counts
    }

    /// gets a reference to word length' count map.
    pub fn len_counts(&self) -> &HashMap<usize,usize> {
        &self.length_map
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
        
        let word_len = word.len();
        let entry = self.length_map.entry(word_len).or_insert(0);
        *entry += 1;

        //consume the word memory, this should be last statement
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

    fn get_sample_solver() -> Result<BoggleBoardSolver, std::io::Error> {
        BoggleBoardSolver::new().with_dictionary("words.txt")
    }

    fn get_sample_board() -> Vec<char> {
        vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S']
    }

    fn solve_sample_board() -> BoggleBoardResult {
        let solver = get_sample_solver().unwrap();
        let board = get_sample_board();

        let result = solver.solve_vec(&board, 4, 4);
        assert!(result.is_some(), "the result does not have value");

        result.unwrap()
    }

    #[test]
    fn sample_solver_loads_successfully(){
        let solver = get_sample_solver();
        assert!(solver.is_ok(), "Failed to load trie from file");
    }

    #[test]
    fn can_solve_sample_board(){
        let _ = solve_sample_board();
    }

    #[test]
    fn there_should_be_atleast_10_words_with_score_3(){
        let result = solve_sample_board();
        let three_count = result.how_many(3);
        assert!(three_count>10);
        println!("There are {three_count} words with score equal to 3");
    }

    #[test]
    fn there_should_be_no_words_with_score_4(){
        let result = solve_sample_board();
        let four_count = result.how_many(4);
        assert_eq!(four_count,0);
        println!("There are {four_count} words with score equal to 4");
    }

    #[test]
    fn there_should_be_some_words_with_3_scores_for_a_board_generated(){
        let solver = get_sample_solver().unwrap();

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

    #[test]
    fn there_should_a_word_with_length_7(){
        let result = solve_sample_board();
        let key = 7;
        assert!(result.len_counts().contains_key(&key));
        println!("There are {} words with length equal to 7", result.len_counts()[&key]);
    }
}