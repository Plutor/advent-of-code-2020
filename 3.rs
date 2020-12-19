fn main() {
    let data = std::fs::read_to_string("3.dat").expect("oops");
    let yslope = 1;
    let xslope = 3;
    let mut x = 0;
    let mut y = 0;

    let mut trees_hit = 0;
    loop {
        let row = data.lines().skip(y).next();
        if row == None {
            break;
        }
        let cell = row.unwrap().chars().skip(x).next().unwrap();
        if cell == '#' {
            trees_hit += 1;
        }
        x = (x + xslope) % row.unwrap().len();
        y += yslope;
    }
    println!("Hit {} trees", trees_hit)
}
