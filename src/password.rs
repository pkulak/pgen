use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::char_class::CharClass;

pub fn generate_password(classes: &Vec<CharClass>, count: u32) -> String {
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