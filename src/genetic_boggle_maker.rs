use genetic_algorithm::strategy::evolve::prelude::*;
use word_trie::trie::Trie;
use crate::boggle_board::Board;
use crate::total_boggle_score_calculator::TotalBoggleScoreCalculator;


#[derive(Clone, Debug)]
struct BoggleFitness<'a> {
    score_calc: TotalBoggleScoreCalculator<'a>,
}

impl <'a> BoggleFitness<'a> {
    pub fn new(dictionary : &'a Trie, x_size:usize, y_size:usize)->Self{
        let score_calc = TotalBoggleScoreCalculator::new(dictionary, x_size, y_size);

        Self{
            score_calc
        }
    }

    fn get_board_score(&mut self,board:&Vec<char>) -> u32{         
        self.score_calc.score(board)
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