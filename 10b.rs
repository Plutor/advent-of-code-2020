fn main() {
    let data = std::fs::read_to_string("10.dat").expect("oops");

    let mut adapters: Vec<usize> = data.lines().map(|s| {s.parse::<usize>().unwrap()}).collect();
    adapters.sort();
    adapters.insert(0,0);
    adapters.push(adapters[adapters.len()-1]+3);

    let mut lasta = 0;
    let mut consec_ones = vec![0];
    for a in adapters {
        let diff = a-lasta;
        println!("{} - {} = {}", a, lasta, diff);
        if diff == 1 {
            *consec_ones.last_mut().unwrap() += 1;
        } else if diff == 3 {
            if consec_ones.last().unwrap() > &0 {
                consec_ones.push(0);
            }
        }
        lasta = a;
    }
    println!("{:?}", consec_ones);
}

