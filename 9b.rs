fn main() {
    let data = std::fs::read_to_string("9.dat").expect("oops");

    let magic = 32321523;
    let mut q = Vec::new();

    for l in data.lines() {
        let num: i64 = l.parse().unwrap();
        q.push(num);
    }

    // [start, start+len)
    let mut start = 0;
    let mut len = 1;
    loop {
        let sum = q.iter().skip(start).take(len).fold(0, |a,b| a+b);
        println!("checking if {} = {}", magic,
                 q.iter().skip(start).take(len).fold(String::new(), |a,b| a + &"+".to_string() + &b.to_string()));
        if sum == magic {
            println!("yes!");
            let mut vals = q.iter().skip(start).take(len).collect::<Vec<&i64>>();
            vals.sort();
            println!("{} + {} = {}", vals[0], vals[vals.len()-1], vals[0] + vals[vals.len()-1]);
            return;
        } else if sum < magic {
            len += 1;
        } else if sum > magic {
            start += 1;
            len -= 1;
        }
    }
}
