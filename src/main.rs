use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let chars = CharClass {
        chars: vec!['a', 'b', 'c'],
        count: 2
    };

    let numbers = CharClass {
        chars: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        count: 0
    };
    
    let pass = generate_password(&vec![chars, numbers], 8);
    println!("{}", pass)
}

fn generate_password(classes: &Vec<CharClass>, count: u32) -> String {
    let mut chosen = Vec::new();

    let required = classes.iter()
        .filter(|c| c.count > 0)
        .collect::<Vec<&CharClass>>();
     
    let optional = classes.iter()
        .filter(|c| c.count == 0)
        .collect::<Vec<&CharClass>>();

    // add in all the required chars
    for c in required.iter() {
        chosen.append(&mut c.pick())
    }

    let mut index = 0;

    // and fill in the rest with the optionals
    while chosen.len() < count as usize {
        let c = optional[index % optional.len()];
        chosen.push(c.pick_one());
        index += 1;
    }


    chosen.shuffle(&mut thread_rng());
    chosen.iter().collect()
}

#[derive(Debug)]
struct CharClass {
    chars: Vec<char>,
    count: u32
}

impl CharClass {
    // randomly choose exactly the required number of chars from this CharClass
    fn pick(&self) -> Vec<char> {
        let mut chosen = Vec::new();

        for _ in 0..self.count {
            chosen.push(self.pick_one());
        }

        chosen
    }

    fn pick_one(&self) -> char {
        let index = thread_rng().gen_range(0, self.chars.len());
        self.chars[index]
    } 
}
