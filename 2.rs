fn main() {
    let data = std::fs::read_to_string("2.dat").expect("oops");

    let mut ok = 0;
    for line in data.lines() {
        let mut iter = line.split_whitespace();
        let nums = iter.next().unwrap();
        let ch = iter.next().unwrap().chars().next().unwrap();
        let pw = iter.next().unwrap();
        let mut iter2 = nums.split("-");
        let lb = iter2.next().unwrap().parse::<i32>().unwrap();
        let ub = iter2.next().unwrap().parse::<i32>().unwrap();

        if pwisok(lb, ub, ch, pw) {
            ok += 1
        }
        println!("{} > {}", line, ok)
    }
    println!("{}", ok)
}

fn pwisok(lb: i32, ub: i32, ch: char, pw: &str) -> bool {
    let mut count = 0;
    for pc in pw.chars() {
        if ch == pc {
            count = count + 1;
        }
    }
    return count >= lb && count <= ub
}

