fn main() {
    let data = std::fs::read_to_string("13.dat").expect("oops");

    let mut lines = data.lines();
    let arrival = lines.next().unwrap().parse::<usize>().unwrap();
    let buses = lines.next().unwrap();
    let mut best = 999999;
    let mut bestid = 0;
    for bus in buses.split(',') {
        if bus == "x" {
            continue;
        }
        let busnum = bus.parse::<usize>().unwrap();
        let sincelast = arrival % busnum;
        let tillnext = busnum - sincelast;
        if tillnext < best {
            best = tillnext;
            bestid = busnum;
        }
        println!("The last bus #{} came {}m ago, {}m till next", busnum, sincelast, tillnext);
    }
    println!("Bus is bus #{}, coming in {}m = {}", bestid, best, best*bestid);
}
