use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use crate::char_class::CharClass;

pub fn generate_password(classes: &[CharClass], count: u32) -> String {
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

    // then each optional once
    for c in optional.iter() {
        chosen.push(c.pick_one())
    }

    let mut index = 0;

    // and fill in the rest more randomly, now that all the requirements
    // are met
    while chosen.len() < count as usize {
        if thread_rng().gen_range(0, 2) == 0 {
            let c = optional[index % optional.len()];
            chosen.push(c.pick_one());
        }
        index += 1;
    }

    chosen.shuffle(&mut thread_rng());
    chosen.iter().collect()
}