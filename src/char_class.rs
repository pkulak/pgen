use rand::Rng;
use rand::thread_rng;

pub fn lower_case() -> Vec<char> {
    "abcdefghjkmnpqrstuvwxyz".chars().collect()
}

pub fn upper_case() -> Vec<char> {
    "ABCDEFGHJKMNPQRSTUVWXYZ".chars().collect()
}

pub fn numbers() -> Vec<char> {
    "1234567890".chars().collect()
}

pub fn symbols() -> Vec<char> {
    ",.?!:'/-".chars().collect()
}

pub fn lower_class() -> CharClass {
    CharClass {
        chars: lower_case(),
        count: 0
    }
}

pub fn upper_class() -> CharClass {
    CharClass {
        chars: upper_case(),
        count: 0
    }
}

pub fn number_class() -> CharClass {
    CharClass {
        chars: numbers(),
        count: 0
    }
}

pub fn symbol_class() -> CharClass {
    CharClass {
        chars: symbols(),
        count: 1
    }
}

pub struct CharClass {
    pub chars: Vec<char>,
    pub count: u32
}

impl CharClass {
    // randomly choose exactly the required number of chars from this CharClass
    pub fn pick(&self) -> Vec<char> {
        let mut chosen = Vec::new();

        for _ in 0..self.count {
            chosen.push(self.pick_one());
        }

        chosen
    }

    pub fn pick_one(&self) -> char {
        let index = thread_rng().gen_range(0, self.chars.len());
        self.chars[index]
    }
}