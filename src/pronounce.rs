use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;

pub fn make_default_graph() -> HashMap<char, Node> {
    let mut graph = HashMap::new();

    graph.insert('a', Node::new('a', "bcdfghjklmnpqrstvwxz"));
    graph.insert('b', Node::new('b', "aeiloruwy"));
    graph.insert('c', Node::new('c', "aehikloru"));
    graph.insert('d', Node::new('d', "aeioruy"));
    graph.insert('e', Node::new('e', "bcdfghijklmnopqrstvwxyz"));
    graph.insert('f', Node::new('f', "aeiloru"));
    graph.insert('g', Node::new('g', "aeiloruwy"));
    graph.insert('h', Node::new('h', "aeiou"));
    graph.insert('i', Node::new('i', "bcdefghjklmnopqrstvwxz"));
    graph.insert('j', Node::new('j', "aeiou"));
    graph.insert('k', Node::new('k', "aeilnoruy"));
    graph.insert('l', Node::new('l', "aeiouy"));
    graph.insert('m', Node::new('m', "aeiou"));
    graph.insert('n', Node::new('n', "aeiouy"));
    graph.insert('o', Node::new('o', "bcdfghijklmnpqrstvwxyz"));
    graph.insert('p', Node::new('p', "aehilnoru"));
    graph.insert('q', Node::new('q', "aeiou"));
    graph.insert('r', Node::new('r', "aeiouy"));
    graph.insert('s', Node::new('s', "acehiklmnopqrtuwy"));
    graph.insert('t', Node::new('t', "aehioruwy"));
    graph.insert('u', Node::new('u', "bcdfgijklmnprstvwxz"));
    graph.insert('v', Node::new('v', "aeioru"));
    graph.insert('w', Node::new('w', "aehiouy"));
    graph.insert('x', Node::new('x', "abcdefghijklmnopqrstuvw"));
    graph.insert('y', Node::new('y', "abcdefghjklmnoprstuw"));
    graph.insert('z', Node::new('z', "aeiouy"));

    graph
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

pub fn generate_password(graph: HashMap<char, Node>, count: u32) -> String {
    let mut chosen = String::new();

    // figure out our starting point
    let keys: Vec<&char> = graph.keys().collect();
    let index = thread_rng().gen_range(0, keys.len());
    let mut node = &graph[&keys[index]];

    while chosen.len() < count as usize {
        let next = node.pick();

        if graph.contains_key(&next) {
            node = &graph[&next];
            chosen.push(next)
        }
    }

    chosen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passwords_are_generated() {
        let password = generate_password(make_default_graph(), 8);
        assert_eq!(password.len(), 8);
    }
}
