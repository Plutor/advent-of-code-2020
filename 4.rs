fn main() {
    let data = std::fs::read_to_string("4.dat").expect("oops");

    let mut valid = 0;
    let mut keyset = std::collections::HashSet::new();
    for line in data.lines() {
        if line == "" {
            if keyset.contains("byr") &&
               keyset.contains("iyr") &&
               keyset.contains("eyr") &&
               keyset.contains("hgt") &&
               keyset.contains("hcl") &&
               keyset.contains("ecl") &&
               keyset.contains("pid") {
                   valid += 1;
            }
            keyset.drain();
        } else {
            for entry in line.split_whitespace() {
                let mut iter = entry.split(':');
                let key = iter.next().unwrap();
                println!("Adding {}", key);
                keyset.insert(key);
            }
        }
    }
    println!("Found {} valid passports", valid)
}
