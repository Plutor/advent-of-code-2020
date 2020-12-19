fn main() {
    let data = std::fs::read_to_string("6.dat").expect("oops");

    let mut sum = 0;
    let mut gp = 0;
    let mut yesset = std::collections::HashMap::new();
    for line in data.lines() {
        if line == "" {
            for (_, p) in yesset.iter() {
                if *p == gp {
                    sum += 1;
                }
            }
            gp = 0;
            yesset.drain();
        } else {
            gp += 1;
            for ch in line.chars() {
                yesset.entry(ch).and_modify(|e|{*e += 1}).or_insert(1);
            }
        }
    }
    println!("{}", sum)
}

