use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

fn main() {
    let data = std::fs::read_to_string("7.dat").expect("oops");

    let mut parentbag: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        let mut iter = line.trim_end_matches('.').split("bags contain");
        let parent = iter.next().unwrap().trim();
        let children = iter.next().unwrap();
        if children == "no other bags" {
            continue;
        }
        for ch in children.split(", ") {
            let desc = ch.split_whitespace()
                     .skip(1).take_while(|d|{*d != "bag" && *d != "bags"})
                     .fold(String::new(), |a, b| a + b + " ").trim_end().to_string();
            println!("{} <= {}", parent, desc);
            parentbag.entry(desc)
                     .or_insert(Vec::new()).push(parent.to_string());
        }
    }

    let mut next_bag: Vec<String> = Vec::new();
    let mut visited = HashSet::new();
    next_bag.push("shiny gold".to_string());
    loop {
        let now = next_bag.pop();
        if now.is_none() {
            break;
        }
        let nowu = now.unwrap();
        visited.insert(nowu.to_string());
        let parents = parentbag.get(&nowu);
        if parents.is_none() {
            continue;
        }
        for next in parents.unwrap().iter() {
            if visited.contains(next) {
                continue;
            }
            next_bag.push(next.to_string());
        }
    }
    println!("{} visited bag colors", visited.len());
}
