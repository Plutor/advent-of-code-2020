use std::collections::HashMap;

fn main() {
    let mut seendata = HashMap::new();
    let data = std::fs::read_to_string("1.dat")
        .expect("file not found!");

    for d in data.lines() {
        let di: i32 = d.parse().unwrap();
        match seendata.get(&(2020-di)) {
            Some(&o) => { println!("Found {} + {} = 2020 :: {} * {} = {}", di, o, di, o, di*o); break; },
            _ => ()
        }
        seendata.insert(di, di);
    }
}
