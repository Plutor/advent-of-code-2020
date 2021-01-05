use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("14.dat").expect("oops");

    let mut clearmask: i64 = 0;
    let mut setmask: i64 = 0;
    let mut mem = HashMap::<i64, i64>::new();

    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();
        let _ = parts.next();
        let val = parts.next().unwrap();
        if cmd == "mask" {
            setmask = 0;
            clearmask = 0;
            for c in val.chars() {
                setmask *= 2;
                clearmask *= 2;
                match c {
                    'X' => {},
                    '1' => {setmask+=1;},
                    '0' => {clearmask+=1;},
                    _ => panic!(),
                }
            }
            println!("mask = {}, clearmask = {}, setmask = {}", val, clearmask, setmask);
        } else {
            let mut ival = val.parse::<i64>().unwrap();
            ival = ival & (!clearmask) | setmask;
            let mut cmdparts = cmd.split(|c| c == '[' || c == ']');
            let _ = cmdparts.next();
            let key = cmdparts.next().unwrap().parse::<i64>().unwrap();
            mem.insert(key, ival);
            println!("wrote {} => {} to {}", val, ival, key);
        }
    }

    let sum = mem.values().fold(0, |acc, v| acc + v);
    println!("Final sum is {}", sum);
}
