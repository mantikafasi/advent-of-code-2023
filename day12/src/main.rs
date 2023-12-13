use lazy_static::lazy_static;
use memoize::memoize;
use pcre2::bytes::Regex;

lazy_static! {
    static ref CONTENTS: String = include_str!("input.txt").to_string();
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    spring: String,
    groups: Vec<u32>,
}

fn main() {
    let map: Vec<Node> = CONTENTS
        .lines()
        .map(|line| {
            let contents = line.split_once(" ").unwrap();
            let mut springs: Vec<char> = contents.0.chars().collect();
            let mut groups: Vec<u32> = contents
                .1
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            groups = groups.repeat(5);
            springs.insert(0, '?');
            springs = springs.repeat(5);
            springs.remove(0);
            Node {
                groups,
                spring: springs.iter().collect(),
            }
        })
        .collect();

    let mut part1 = 0;

    for (i, line) in map.iter().enumerate() {
        let res = search(line.clone());
        println!("On line {}: {}", i, res);
        part1 += res;
    }

    println!("Part 1: {}", part1);
}

#[memoize]
fn search(node: Node) -> usize {
    if node.groups.len() == 0 {
        if node.spring.contains("#") {
            return 0;
        }
        return 1;
    }

    let group = node.groups[0];

    let re = Regex::new(&format!(r"(?=((?:#|\?){{{}}}))", group)).unwrap();
    let matches = re.captures_iter(node.spring.as_bytes());

    let mut res = 0;

    for rematch in matches {
        let mut new_node = node.clone();

        let rematch = rematch.unwrap().get(1).unwrap();

        // println!("{:?} ", rematch);
        if new_node.spring.chars().nth(rematch.start() - 1).unwrap_or(' ') == '#'
            || new_node.spring.chars().nth(rematch.end()).unwrap_or(' ') == '#'
        { continue; }

        let extra = if new_node
            .spring
            .chars()
            .nth(rematch.end())
            .unwrap_or(' ') == ' ' { 0 } else { 1 };
                    
        if new_node.spring[0..rematch.start()].contains('#') {
            continue;
        }

        new_node.spring = new_node.spring[rematch.end() + extra..].to_string();

        new_node.groups.remove(0);
        res += search(new_node);
    }
    return res;
}
