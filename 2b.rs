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
    
    let lbc = pw.chars().skip(lb as usize-1).next().unwrap();
    let ubc = pw.chars().skip(ub as usize-1).next().unwrap();

    return (lbc == ch) != (ubc == ch);
}

