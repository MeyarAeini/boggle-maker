use std::collections::HashSet;
use crate::boggle_dfs::{WordVisitor,BoggleDfsContext,BoggleDfs,get_word_score};
use word_trie::trie::Trie;

struct TotalScoreWordVisitor(HashSet<String>,u32);

impl TotalScoreWordVisitor {
    pub fn score(&self) -> u32 {
        self.1
    }
}

impl WordVisitor for TotalScoreWordVisitor {
    fn visit(&mut self, word: &str){
        if !self.0.contains(word) {
            self.1 += get_word_score(word);
            self.0.insert(word.to_string());
        }
    }
}

#[derive(Debug, Clone)]
pub struct TotalBoggleScoreCalculator<'a> (BoggleDfsContext<'a>);

impl <'a> TotalBoggleScoreCalculator<'a> {
    pub fn new(dictionary : &'a Trie, width:usize, length:usize) -> Self{
        Self(BoggleDfsContext::new(dictionary, width, length))
    }

    pub fn score(&mut self, board: & Vec<char>) -> u32{
        if board.len() != self.0.count() {
            panic!("The board size must be fit to the length:{0} and width:{1}", self.0.length(), self.0.width());
        }
       
        let mut visitor = TotalScoreWordVisitor(HashSet::new(),0);
        let mut session = BoggleDfs::new(&self.0 , board);
        session.with_visitor(&mut visitor).search();

        visitor.score()
    }  
}