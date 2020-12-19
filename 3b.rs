fn main() {
    println!("{}", trees(1,1) *
                   trees(3,1) *
                   trees(5,1) *
                   trees(7,1) *
                   trees(1,2))
}

fn trees(xslope: usize, yslope: usize) -> usize{
    let data = std::fs::read_to_string("3.dat").expect("oops");
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
    println!("({}, {}) => hit {} trees", xslope, yslope, trees_hit);
    return trees_hit
}
