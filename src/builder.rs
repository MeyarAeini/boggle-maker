use crate::genetic_boggle_maker;
use crate::boggle_board::Board;
use word_trie::TrieBuilder;

///The boggle board builder struct
#[derive(Default)]
pub struct BoggleBuilder {
    width : Option<usize>,
    length : Option<usize>,
    target_score: Option<isize>,
    dictionary_path: Option<String>,
}

impl BoggleBuilder {
    ///initiates a new instance of the boggle board builder
    pub fn new() -> Self {
        Self::default()
    }

    ///sets the trie dictionary text file path
    pub fn with_dictionary_path<P: Into<String>>(mut self, path: P) -> Self {
        self.dictionary_path = Some(path.into());
        self
    } 

    ///sets the desired target score for the board to be generated
    pub fn with_target_score(mut self, score: isize) -> Self {
        self.target_score = Some(score);
        self
    } 

    ///sets the board's width
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    } 

    ///sets the board's length
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    } 

    ///generates a boggle board instance
    ///
    /// # Examples
    ///
    /// ```
    /// use boggle_maker::BoggleBuilder;
    /// let builder = BoggleBuilder::new()
    ///.with_dictionary_path("words.txt")
    ///.with_target_score(2500)
    ///.with_length(4)
    ///.with_width(4);
    ///
    ///if let Some(board) = builder.build().expect("Failed to load trie from words.txt file"){
    ///   assert!(board.score().unwrap()>=2500);
    ///}
    ///else{
    ///assert!(false);
    ///}
    /// ```
    pub fn build(self) -> Result<Option<Board>, std::io::Error> {
        let width = match self.width {
            Some(w) => w,
            None => 4,
        };

        let length = match self.length {
            Some(l) => l,
            None => 4,
        };

        let target_score = match self.target_score {
            Some(score) => score,
            None => 0,
        };

        if let Some(path) = self.dictionary_path {
            let trie = TrieBuilder::new()
            .from_file(path)
            .expect("Failed to load trie from file");

            let board = genetic_boggle_maker::make(width,length,target_score,&trie);
            return Ok(board);
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn should_generate_a_board_with_3600_score(){
        should_generate_a_board_with_deired_score(3200);
    }

    #[test]
    fn should_generate_a_board_with_2500_score(){
        should_generate_a_board_with_deired_score(2500);
    }

    fn should_generate_a_board_with_deired_score(score:isize){
        let builder = BoggleBuilder::new()
         .with_dictionary_path("words.txt")
         .with_target_score(score)
         .with_length(4)
         .with_width(4);
        
         if let Some(board) = builder.build().expect("Failed to load trie from words.txt file"){
            assert!(board.score().unwrap()>=score);
         }
         else{
         assert!(false);
        }
    }
}