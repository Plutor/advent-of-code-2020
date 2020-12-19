fn main() {
    let data = std::fs::read_to_string("6.dat").expect("oops");

    let mut sum = 0;
    let mut yesset = std::collections::HashSet::new();
    for line in data.lines() {
        if line == "" {
            sum += yesset.len();
            yesset.drain();
        } else {
            for ch in line.chars() {
                yesset.insert(ch);
            }
        }
    }
    println!("{}", sum)
}

