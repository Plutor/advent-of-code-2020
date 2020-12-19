use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let data = std::fs::read_to_string("7.dat").expect("oops");

    let mut childbag: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        let mut iter = line.trim_end_matches('.').split("bags contain");
        let parent = iter.next().unwrap().trim();
        let children = iter.next().unwrap().trim();
        if children == "no other bags" {
            continue;
        }
        for ch in children.split(", ") {
            let count:usize = ch.split_whitespace().next().unwrap().trim().parse().unwrap();
            let desc = ch.split_whitespace()
                     .skip(1).take_while(|d|{*d != "bag" && *d != "bags"})
                     .fold(String::new(), |a, b| a + b + " ").trim_end().to_string();
            println!("{} <= {} * {}", parent, desc, count);
            for _ in 0..count {
                childbag.entry(parent.to_string())
                         .or_insert(Vec::new()).push(desc.to_string());
            }
        }
    }

    println!("{} child bags", childbagcount(&childbag, "shiny gold".to_string()))
}

fn childbagcount(childbag: &HashMap<String, Vec<String>>, root: String) -> usize {
    let mut count = 1;
    let ch = childbag.get(&root);
    if ch.is_none() {
        return count;
    }
    for next in ch.unwrap().iter() {
        count += childbagcount(childbag, next.to_string());
    }
    return count
}
