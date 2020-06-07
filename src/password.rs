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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char_class::{CharClass, lower_class, number_class, symbol_class, upper_class};

    #[test]
    fn passwords_are_generated() {
        let all = vec![lower_class(), upper_class(), number_class(), symbol_class()];

        for (i, class) in all.iter().enumerate() {
            let password = generate_password(&all[0..(i + 1)], 8);
            assert_eq!(password.len(), 8);
            assert!(string_contains_class(&password, &class));
        }
    }

    fn string_contains_class(s: &String, class: &CharClass) -> bool {
        let mut count = 0;

        for c in class.chars.iter() {
            if s.contains(|l| *c == l) {
                count += 1;
            }
        }

        // it has to be present at least once
        if count == 0 {
            return false;
        }

        // if the class doesn't require a specific amount, we're done
        if class.count == 0 {
            return true;
        }

        // otherwise, we need to be more strict
        class.count == count
    }
}
