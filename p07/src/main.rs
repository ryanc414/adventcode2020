use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = load_input();

    let &root = input.bags_by_name.get("shiny gold").unwrap();

    let num_parents = input.find_parents(root).len();
    println!("shiny gold bag can be contained by {} others", num_parents);

    let num_children = input.count_children(root);
    println!("shiny gold bag must contain {} other bags", num_children);
}

struct BagGraph {
    bags: Vec<Bag>,
    bags_by_name: HashMap<String, usize>,
}

impl BagGraph {
    fn parse_children(
        line: &str,
        children_re: &Regex,
        bag: &mut Bag,
        bag_names: &HashMap<String, usize>,
    ) {
        if line == "no other bags" {
            return;
        }

        bag.children = line
            .split(", ")
            .map(|item: &str| -> (usize, usize) {
                let caps = children_re.captures(item).unwrap();
                let num: usize = caps[1].parse().unwrap();
                let name = &caps[2];
                (num, *bag_names.get(name).unwrap())
            })
            .collect();
    }

    fn find_parents(&self, node_ix: usize) -> HashSet<usize> {
        let mut parents: HashSet<usize> = self.bags[node_ix].parents.iter().cloned().collect();
        for &i in self.bags[node_ix].parents.iter() {
            parents.extend(self.find_parents(i));
        }
        parents
    }

    fn count_children(&self, node_ix: usize) -> usize {
        let mut count = 0;

        for &(child_count, child_ix) in self.bags[node_ix].children.iter() {
            count += child_count * (1 + self.count_children(child_ix));
        }

        count
    }
}

impl<'a> FromIterator<&'a str> for BagGraph {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let line_re = Regex::new(r"^([a-z ]+) bags contain ([a-z0-9, ]+)\.$").unwrap();
        let (mut bags, child_text): (Vec<Bag>, Vec<String>) = iter
            .into_iter()
            .map(|line: &str| -> (Bag, String) {
                let caps = line_re.captures(line).unwrap();
                (
                    Bag {
                        name: caps[1].to_string(),
                        children: Vec::new(),
                        parents: Vec::new(),
                    },
                    caps[2].to_string(),
                )
            })
            .unzip();

        let bag_map: HashMap<String, usize> = bags
            .iter()
            .enumerate()
            .map(|(i, bag)| (bag.name.clone(), i))
            .collect();

        let children_re = Regex::new(r"^(\d+) ([a-z ]+) bags?$").unwrap();
        for (i, line) in child_text.iter().enumerate() {
            Self::parse_children(line, &children_re, &mut bags[i], &bag_map);
        }

        for i in 0..bags.len() {
            let children: Vec<(usize, usize)> = bags[i].children.to_vec();
            for (_, child_ix) in children {
                bags[child_ix].parents.push(i);
            }
        }

        Self {
            bags,
            bags_by_name: bag_map,
        }
    }
}

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    children: Vec<(usize, usize)>,
    parents: Vec<usize>,
}

fn load_input() -> BagGraph {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("please specify input filename");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("error reading the file");

    contents
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect()
}
