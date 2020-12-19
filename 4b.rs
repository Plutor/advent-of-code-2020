// rustc 4b.rs -L libs && ./4b 

extern crate regex;
use regex::Regex;

fn main() {
    let data = std::fs::read_to_string("4.dat").expect("oops");
    let year_re = Regex::new(r"^[0-9]{4}$").unwrap();
    let hgt_re = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();

    let mut valid = 0;
    let mut keyset = std::collections::HashSet::new();
    for line in data.lines() {
        if line == "" {
            if !keyset.contains("byr") {
                println!("  no byr");
            } else if !keyset.contains("iyr") {
                println!("  no iyr");
            } else if !keyset.contains("eyr") {
                println!("  no eyr");
            } else if !keyset.contains("hgt") {
                println!("  no hgt");
            } else if !keyset.contains("hcl") {
                println!("  no hcl");
            } else if !keyset.contains("ecl") {
                println!("  no ecl");
            } else if !keyset.contains("pid") {
                println!("  no pid");
            } else {
                valid += 1;
            }
            keyset.drain();
            println!("  =================");
        } else {
            for entry in line.split_whitespace() {
                println!("Trying {}", entry);
                let mut iter = entry.split(':');
                let key = iter.next().unwrap();
                let val = iter.next().unwrap();
                let valid_key = match key {
                    "byr" => year_re.is_match(val) &&
                             val.parse::<i32>().unwrap() >= 1920 &&
                             val.parse::<i32>().unwrap() <= 2002,
                    "iyr" => year_re.is_match(val) &&
                             val.parse::<i32>().unwrap() >= 2010 &&
                             val.parse::<i32>().unwrap() <= 2020,
                    "eyr" => year_re.is_match(val) && 
                             val.parse::<i32>().unwrap() >= 2020 &&
                             val.parse::<i32>().unwrap() <= 2030,
                    "hgt" => {
                        let cap = hgt_re.captures(val);
                        if cap.is_none() {
                            false
                        } else {
                            let cu = cap.unwrap();
                            let num = cu[1].parse::<i32>().unwrap();
                            println!("{:?}", cu);
                            match &cu[2] {
                                "cm" => num >= 150 && num <= 193,
                                "in" => num >= 59 && num <= 76,
                                _ => false
                            }
                        }
                    },
                    "hcl" => hcl_re.is_match(val),
                    "ecl" => ecl_re.is_match(val),
                    "pid" => pid_re.is_match(val),
                    _ => false
                };
                println!("  valid_key = {}", valid_key);
                if valid_key {
                    keyset.insert(key);
                }
            }
        }
    }
    println!("Found {} valid passports", valid)
}
