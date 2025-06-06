use word_trie::trie::{Trie,TrieNode};

/// gets the word score 
pub fn get_word_score(word: &str) -> u32{
    let len = word.len();
    match len {
        3 => 1,
        4 => 1,
        5 => 2,
        6 => 3,
        7 => 5,
        _ if len >= 8 => 11,
        _ => 0,
    }
}

/// The boggle dfs context struct.
#[derive(Debug, Clone)]
pub struct BoggleDfsContext<'a> {
    dictionary: &'a Trie,
    length: usize,
    width: usize,
}

impl <'a> BoggleDfsContext<'a> {
    /// initiate a new boggle dfs context.
    pub fn new(dictionary : &'a Trie, width:usize, length:usize)->Self{
        Self{
            dictionary,
            length,
            width,
        }
    }

    /// get boggle board's width
    pub fn width(&self) -> usize {
        self.width
    }

    /// get boggle board's length
    pub fn length(&self) -> usize {
        self.length
    }

    /// gets the trie for the boggle board dfs
    pub fn dictionary(&self) -> &'a Trie {
        self.dictionary
    }

    /// gets the boggle board characters count.
    pub fn count(&self) -> usize {
        self.width * self.length
    }
}

/// The word visitor trait 
pub trait WordVisitor {
    fn visit(&mut self, word: &str, path: &Vec<u16>);    
}

/// The boggle DFS struct
pub struct BoggleDfs<'a> {    
    context: &'a BoggleDfsContext<'a>,
    visitors: Vec<&'a mut dyn WordVisitor>,
    visited: Vec<bool>,
    current: String,
    board: &'a Vec<char>,
    path: Vec<u16>,
}

impl<'a> BoggleDfs<'a>{
    ///initiate a new boggle dfs instance
    pub fn new(context : &'a BoggleDfsContext<'a>,board: &'a Vec<char>) -> Self {
        let visited = vec![false; context.count()];
        let current = String::new();
        let path = Vec::new();
        Self{
            context,
            visitors: Vec::new(),
            visited,
            current,
            board,
            path,
        }
    }

    /// add a visitor to be triggered on dfs result found events
    pub fn with_visitor(&mut self, visitor: &'a mut dyn WordVisitor) -> &mut Self{
        self.visitors.push(visitor);

        self
    }

    /// trigger the dfs search
    pub fn search(&mut self){   
        for i in 0..self.context.width {
            for j in 0..self.context.length {
                self.dfs(&self.context.dictionary().root, i, j);
            }
        }
    }

    fn dfs(&mut self,mut node: &TrieNode ,x: usize,y: usize)
    {        
        let cell_index = x * self.context.width() + y;
        //if the board's current cell is visited then return.
        if self.visited[cell_index] {
            return;
        }
        
        //mark the current board's cell as visited
        self.visited[cell_index] = true;
        let ch = self.cell_value(cell_index);
        
        //check if the trie has this node
        let ch_index = ch.to_ascii_lowercase();        
        match node.nodes.get(&ch_index){
            Some(next_node) => node = next_node,
            None => {
                //if the trie current node does not have the board's current cell's letter then revert the status and return 
                self.visited[cell_index] = false;
                return;
            },
        }
        //add current cell's letter to the current word
        self.current.push(ch);
        self.path.push(cell_index as u16);

        //check if the current word is a valid word in the dictionary 
        if node.is_word { 
            self.visit();    
        }

        //Recursively check all neighbour cells 
        for (a,b) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]{
            let next_x = x as i8 + a;
            let next_y = y as i8 + b;
            if (0..self.context.width() as i8).contains(&next_x) && (0..self.context.length() as i8).contains(&next_y){
                self.dfs(node, next_x as usize, next_y as usize); 
            }           
        }

        //Remove current visited cell letter from current word end.
        self.current.pop();
        self.path.pop();

        //mark the current cell as not visited 
        self.visited[cell_index] = false;      
    }

    fn visit(&mut self) {
        // this part needs to be enhanced to use async 
        for visitor in self.visitors.iter_mut() {
            visitor.visit(&self.current, &self.path);    
        } 
    }

    fn cell_value(&self, index: usize) -> char {
        self.board[index]
    }
}