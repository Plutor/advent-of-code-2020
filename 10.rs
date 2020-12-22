fn main() {
    let data = std::fs::read_to_string("10.dat").expect("oops");

    let mut adapters: Vec<usize> = data.lines().map(|s| {s.parse::<usize>().unwrap()}).collect();
    adapters.sort();

    let mut lasta = 0;
    let mut diffs = vec![0; 4];
    for a in adapters {
        println!("{} - {} = {}", a, lasta, a-lasta);
        diffs[a-lasta] += 1;
        lasta = a;
    }
    println!("{:?}", diffs);
    println!("{}", diffs[1]*(diffs[3]+1));
}

