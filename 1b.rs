use std::collections::HashMap;

fn main() {
    let mut seendata = HashMap::new();
    let data = std::fs::read_to_string("1.dat")
        .expect("file not found!");

    for d in data.lines() {
        let di: i32 = d.parse().unwrap();
        for (e,_) in &seendata {
            match seendata.get(&(2020-di-e)) {
                Some(&o) => {
                    println!("Found {} + {} + {} = 2020 :: {} * {} * {} = {}", di, e, o, di, e, o, di*e*o);
                    break; },
                _ => ()
            }
        }
        seendata.insert(di, di);
    }
}
