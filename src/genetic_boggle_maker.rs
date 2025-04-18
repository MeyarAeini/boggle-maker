use genetic_algorithm::strategy::evolve::prelude::*;
use std::collections::HashMap;
use word_trie::trie::{Trie,TrieNode};
use crate::boggle_board::Board;


#[derive(Clone, Debug)]
struct BoggleFitness<'a> {
    dictionary: &'a Trie,
    x_size : usize,
    y_size : usize,
}

impl <'a> BoggleFitness<'a> {
    pub fn new(dictionary : &'a Trie, x_size:usize, y_size:usize)->Self{
        Self{
            dictionary,
            x_size,
            y_size,
        }
    }

    fn get_board_score(&self,board:&Vec<char>) -> usize{         
        let mut set = HashMap::new();
        let mut visited = HashMap::new(); 

        let mut current = String::new();

        let mut word_found_handler = |word: &str| {
            //if yes then check if it's not been added to the result set yet
            //if not added then add it to the result set with a proper calculated score
            if !set.contains_key(word) {               
                set.insert(word.to_string(), Self::get_score(word.len()));
            }
        };

        for i in 0..self.x_size{
            for j in 0..self.y_size{
                self.get_board_score_from(
                    &self.dictionary.root,
                    board,
                    &mut visited,
                    i,
                    j,
                    &mut current,   
                    &mut word_found_handler                 
                    );
            }
        }

        let mut score = 0;

        for (_, val) in &set {
            score += val;
        }

        score
    }

    fn get_board_score_from<F>(&self,mut node:&TrieNode ,
            brd:&Vec<char>,
            visited:&mut HashMap<usize,bool>,
            x:usize,
            y:usize,
            current:&mut String,
            on_word_found: &mut F
        )
        where F: FnMut(&str)
    {
        
        let cell_index = x*self.x_size + y;
        let is_visited = visited.entry(cell_index).or_insert(false);
        //if the board's current cell is visited then return.
        if *is_visited {
            return;
        }
        
        //mark the current board's cell as visited
        *is_visited = true;
        let ch = brd[cell_index];//brd.get(x,y).expect("the board must has value in all cells");
        
        //check if the trie has this node
        let ch_index = ch.to_ascii_lowercase();        
        match node.nodes.get(&ch_index){
            Some(next_node) => node = next_node,
            None => {
                //if the trie current node does not have the board's current cell's letter then revert the status and return 
                visited.insert(cell_index,false);
                return;
            },
        }
        //add current cell's letter to the current word
        current.push(ch);

        //check if the current word is a valid word in the dictionary 
        if node.is_word {   
            on_word_found(current);             
        }

        //Recursively check all neighbour cells 
        for (a,b) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]{
            let next_x = x as i8 + a;
            let next_y = y as i8 + b;
            if (0..self.x_size as i8).contains(&next_x) && (0..self.y_size as i8).contains(&next_y){
                self.get_board_score_from(node,brd, visited,next_x as usize,next_y as usize, current, on_word_found); 
            }           
        }

        //Remove current visited cell letter from current word end.
        current.pop();

        //mark the current cell as not visited 
        visited.insert(cell_index,false);        
    }

    fn get_score(len:usize)->usize{
        match len{
            0..=2 => 0,
            3..=4 => 1,
            5 => 2,
            6 => 3,
            7 => 5,
            _ => 11
        }
    }
}

impl Fitness for BoggleFitness<'_> {
    type Genotype = ListGenotype<char>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {

        let score = self.get_board_score(&chromosome.genes);
        Some(score as FitnessValue)
    }

    
}

///generates a boggle board instance by given width, length, target score 
/// and a trie to be used for calculation of the scores
///this method is using genetic algorithm for finding the best fit for the target score
pub fn make(width : usize, length : usize, target_score: isize , dictionary: &Trie) 
-> Option<Board> {
    let allele_lists = vec![
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];

    //define the geno type
    let genotype = ListGenotype::builder()
     .with_genes_size(width * length)
     .with_allele_list(allele_lists)
     .with_genes_hashing(false) // optional, defaults to false
     .build()
     .unwrap();

     let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(25)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectElite::new(0.8))
        .with_extension(ExtensionMassDegeneration::new(2, 10))
         .with_reporter(EvolveReporterSimple::new(50))
        // .with_par_fitness(true)
        .with_target_fitness_score(target_score)
        //.with_select(SelectTournament::new(4, 0.9))
        //.with_mutate(MutateMultiGeneDynamic::new(2, 0.1, 250))
        //.with_reporter(EvolveReporterDuration::new())
        .with_fitness(BoggleFitness::new(dictionary,width,length));

    let (evolve, _) = evolve_builder.call_speciated(10).unwrap();
        if let Some(best_chromosome) = evolve.best_chromosome() {
        let board = Board::new(best_chromosome.genes, width, length, best_chromosome.fitness_score);

        Some(board)
        //println!("{:?}",best_chromosome.genes);
    } else {
        //println!("Invalid solution with fitness score: None");
        None
    }
}