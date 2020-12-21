fn main() {
    let data = std::fs::read_to_string("9.dat").expect("oops");

    let mut q = Vec::new();

    for l in data.lines() {
        let num: i64 = l.parse().unwrap();
        
        if q.len() == 25 {
            let mut ok = false;
            for a in 0..q.len() {
                for b in a+1..q.len() {
                    if q[a] + q[b] == num {
                        println!("{} = {} + {}", num, q[a], q[b]);
                        ok = true;
                        break;
                    }
                }
            }

            if !ok {
                println!("{} is a bad value", num);
                break;
            }
        }

        q.insert(0, num);
        if q.len() > 25 {
            q.pop();
        }
    }
}
