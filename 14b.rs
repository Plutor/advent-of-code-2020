use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("14.dat").expect("oops");

    let mut floats: Vec<usize> = Vec::new();
    let mut clearmask: i64 = 0;
    let mut setmask: i64 = 0;
    let mut mem = HashMap::<i64, i64>::new();

    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap();
        let _ = parts.next();
        let val = parts.next().unwrap();
        if cmd == "mask" {
            clearmask = 0;
            setmask = 0;
            floats.clear();
            for (idx, c) in val.chars().enumerate() {
                clearmask *= 2;
                setmask *= 2;
                match c {
                    'X' => {clearmask+=1;floats.push(val.len()-idx-1);},
                    '1' => {setmask+=1;},
                    '0' => {},
                    _ => panic!(),
                }
            }
            println!("mask = {}, floats = {:?}, setmask = {}", val, floats, setmask);
        } else {
            let ival = val.parse::<i64>().unwrap();
            let mut cmdparts = cmd.split(|c| c == '[' || c == ']');
            let _ = cmdparts.next();
            let key = cmdparts.next().unwrap().parse::<i64>().unwrap() & !clearmask | setmask;
            for n in 0..2i64.pow(floats.len() as u32) {
                let mut ikey = key;
                for (di, d) in floats.iter().enumerate() {
                    let bit = ((n >> di) & 1) << d;
                    println!("n={}, di={}, d={}, bit={}", n, di, d, bit);
                    ikey = ikey | bit;
                }
                mem.insert(ikey, ival);
            }
        }
    }

    let sum = mem.values().fold(0, |acc, v| acc + v);
    println!("Final sum is {}", sum);
}
