use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;

pub fn make_default_tree() -> HashMap<char, Node> {
    let mut tree = HashMap::new();

    tree.insert('a', Node::new('a', "bcdfghjklmnpqrstvwxz"));
    tree.insert('b', Node::new('b', "aeiloruwy"));
    tree.insert('c', Node::new('c', "aehikloru"));
    tree.insert('d', Node::new('d', "aeioruy"));
    tree.insert('e', Node::new('e', "bcdfghijklmnopqrstvwxyz"));
    tree.insert('f', Node::new('f', "aeiloru"));
    tree.insert('g', Node::new('g', "aeiloruwy"));
    tree.insert('h', Node::new('h', "aeiou"));
    tree.insert('i', Node::new('i', "bcdefghjklmnopqrstvwxz"));
    tree.insert('j', Node::new('j', "aeiou"));
    tree.insert('k', Node::new('k', "aeilnoruy"));
    tree.insert('l', Node::new('l', "aeiouy"));
    tree.insert('m', Node::new('m', "aeiou"));
    tree.insert('n', Node::new('n', "aeiouy"));
    tree.insert('o', Node::new('o', "bcdfghijklmnpqrstvwxyz"));
    tree.insert('p', Node::new('p', "aehilnoru"));
    tree.insert('q', Node::new('q', "aeiou"));
    tree.insert('r', Node::new('r', "aeiouy"));
    tree.insert('s', Node::new('s', "acehiklmnopqrtuwy"));
    tree.insert('t', Node::new('t', "aehioruwy"));
    tree.insert('u', Node::new('u', "bcdfgijklmnprstvwxz"));
    tree.insert('v', Node::new('v', "aeioru"));
    tree.insert('w', Node::new('w', "aehiouy"));
    tree.insert('x', Node::new('x', "abcdefghijklmnopqrstuvw"));
    tree.insert('y', Node::new('y', "abcdefghjklmnoprstuw"));
    tree.insert('z', Node::new('z', "aeiouy"));

    tree
}

pub struct Node {
    pub symbol: char,
    pub children: Vec<char>
}

impl Node {
    pub fn pick(&self) -> char {
        let index = thread_rng().gen_range(0, self.children.len());
        self.children[index]
    }

    pub fn new(symbol: char, children: &str) -> Node {
        Node {
            symbol,
            children: children.chars()
                .map(|c| c.to_owned())
                .collect()
        }
    }
}

pub fn generate_password(map: HashMap<char, Node>, count: u32) -> String {
    let mut chosen = String::new();

    // figure out our starting point
    let keys: Vec<&char> = map.keys().collect();
    let index = thread_rng().gen_range(0, keys.len());
    let mut node = &map[&keys[index]];

    while chosen.len() < count as usize {
        let next = node.pick();

        if map.contains_key(&next) {
            node = &map[&next];
            chosen.push(next)
        }
    }

    chosen
}
