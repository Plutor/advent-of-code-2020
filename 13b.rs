fn main() {
    let data = std::fs::read_to_string("13.dat").expect("oops");

    let mut lines = data.lines();
    let _ = lines.next();
    let buses = lines.next().unwrap().split(',');
    let mut jump: i64 = 0;
    let mut cur: i64 = 0;
    for (n, bus) in buses.enumerate() {
        if bus == "x" {
            continue;
        }
        let busnum = bus.parse::<i64>().unwrap();
        let mut target: i64 = (busnum - n as i64) % busnum;
        while target < 0 {
            target += busnum;
        }
        println!("busnum #{} at index {}, want (time%{})={}", busnum, n, busnum, target);
        if jump == 0 {
            jump = busnum;
            println!("jumping by {}", jump);
        } else {
            loop {
                if cur % busnum == target {
                    break;
                }
                cur += jump;
            }
            jump = lcm(jump, busnum);
            println!("found at {}, now jumping by {}", cur, jump);
        }
    }
    println!("done at {}", cur);
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}
fn gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 { return a; }
        let na = b;
        let nb = a%b;
        a = na;
        b = nb;
    }
}
