use std::collections::HashSet;
use crate::boggle_dfs::{WordVisitor,BoggleDfsContext,BoggleDfs,get_word_score};
use word_trie::trie::Trie;

struct BoggleBoardSolverVisitor {
    words: HashSet<String>,
    counts: Vec<u32>,
}

impl BoggleBoardSolverVisitor {
    pub fn new() -> Self {
        Self{
            words: HashSet::new(),
            counts : vec![0;12], 
        }
    }

    pub fn how_many(&self, score: usize) -> u32 {        
        if score > 11 { 0 } else { self.counts[score] }        
    }

    pub fn words(&self) -> &HashSet<String> {
        &self.words
    }

    fn score_guard(score: usize){
        if score > 12 {
            panic!("the functionality has not been implemented for score bigger than 11");
        }
    }
}

impl WordVisitor for BoggleBoardSolverVisitor {
    fn visit(&mut self, word: &str){
        if self.words.contains(word) {
            return;
        }

        //add the word to visited list
        self.words.insert(word.to_string());

        //update the score map
        let score = get_word_score(word);

        Self::score_guard(score as usize);

        self.counts[score as usize] += 1;
    }
}

pub struct BoggleBoardSolver<'a> (BoggleDfsContext<'a>,BoggleBoardSolverVisitor); 

impl <'a> BoggleBoardSolver<'a> {
    pub fn new(dictionary : &'a Trie, width:usize, length:usize) -> Self{
        Self(BoggleDfsContext::new(dictionary, width, length), BoggleBoardSolverVisitor::new())
    }

    pub fn solve(&mut self, board: &Vec<char>){
        if board.len() != self.0.count() {
            panic!("The board size must be fit to the length:{0} and width:{1}", self.0.length(), self.0.width());
        }

        BoggleDfs::new(&self.0, board).with_visitor(&mut self.1).search();
    }

    pub fn how_many(&self, score: usize) -> u32 {
        self.1.how_many(score)
    }

    pub fn words(&self) -> &HashSet<String> {
        self.1.words()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use word_trie::TrieBuilder;

    #[test]
    fn there_should_be_atleast_10_words_with_score_3(){
        let trie = TrieBuilder::new()
            .from_file("words.txt")
            .expect("Failed to load trie from file");

        let mut solver = BoggleBoardSolver::new(&trie, 4, 4);
        let board = vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S'];

        solver.solve(&board);

        let three_count = solver.how_many(3);
        assert!(three_count>10);
        println!("There are {three_count} words with score equal to 3");
    }

    #[test]
    fn there_should_be_atleast_0_words_with_score_4(){
        let trie = TrieBuilder::new()
            .from_file("words.txt")
            .expect("Failed to load trie from file");

        let mut solver = BoggleBoardSolver::new(&trie, 4, 4);
        let board = vec!['S','E','R','S','P','A','T','G','L','I','N','E','S','E','R','S'];

        solver.solve(&board);

        let four_count = solver.how_many(4);
        assert_eq!(four_count,0);
        println!("There are {four_count} words with score equal to 4");
    }
}