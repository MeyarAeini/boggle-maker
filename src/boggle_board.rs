use rand::distr::{Distribution, Uniform};

///The board struct
pub struct Board{
    width : usize,
    length : usize,
    value : Vec<char>,
    score : Option<isize>
}

impl Board{
    
    ///returns new board instance by given char vector and board width, length and score
    pub fn new(value : Vec<char>, width : usize ,length : usize, score: Option<isize>) -> Self{
        Self{
            width : width,
            length : length,
            value : value,
            score : score,
        }
    }

    ///gets board's width
    pub fn width(&self) -> usize{
        self.width
    }

    ///gets board's length
    pub fn length(&self) -> usize{
        self.length
    }

    ///gets board's score
    pub fn score(&self) -> Option<isize>{
        self.score
    }

    /// gets board's value
    pub fn value(&self) -> &Vec<char>{
        &self.value
    }

    ///generate a random board by given width and length
    pub fn new_random(width : usize, length : usize)->Self{
        let mut brd = Self{
            width : width,
            length : length,
            value : vec![' '; width * length],
            score : None
        };

        let uniform = Uniform::try_from(0..26).unwrap();
        let mut rng = rand::rng();

        for i in 0..width*length {
            let rnd = uniform.sample(&mut rng);
            let random_char = (b'A' + rnd) as char;
            brd.value[i]= random_char;
        }
        
        
        brd
    }

    pub fn copy(&self) -> Self{
        Self{
            width : self.width,
            length : self.length,
            value : self.value.to_vec(),
            score : self.score,
        }
    }

    pub fn to_string(&self)->String{
        let mut s = String::new();
        for (i,val) in self.value.iter().enumerate() {
            if i>0 && i%self.width == 0{
                s.push('\n');
            }
            s.push(*val);
        }

        s
    }

    pub fn hash(&self)->String{
        let mut s = String::new();
        for val in self.value.iter() {
            s.push(*val);
        }

        s
    }

    ///gets the char located in (i,j)
    pub fn get(&self, i:usize,j:usize)->Option<char>{
        if i>=self.width || j>=self.length
        {
            return None;
        }
        Some(self.value[i* self.width + j])
    }  
    
    ///sets char located in (i,j)
    pub fn set(&mut self, i:usize,j:usize,ch:char){
        if !(i>=self.width || j>=self.length)
        {
            self.value[i* self.width + j] = ch;
        }
    }
}