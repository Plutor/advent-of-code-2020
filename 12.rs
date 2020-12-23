fn main() {
    let data = std::fs::read_to_string("12.dat").expect("oops");

    let mut facing = 'E';
    let (mut x, mut y) = (0,0);
    for line in data.lines() {
        let mut chars = line.chars();
        let mut instr = chars.next().unwrap();
        let dist: i32 = chars.fold(String::new(), |a,c|{a+&c.to_string()}).parse().unwrap();

        if instr == 'F' { instr = facing; }
        match instr {
            'N' => y -= dist,
            'S' => y += dist,
            'E' => x += dist,
            'W' => x -= dist,
            'R' => facing = turnby(facing, dist/90),
            'L' => facing = turnby(facing, 4-dist/90),
            _ => return,
        }
        println!("Move {} by {}, facing {}, now at ({},{})", instr, dist, facing, x, y);
    }
}

fn turnby(dir: char, steps: i32) -> char{
    let dirs = ['N', 'E', 'S', 'W'];
    println!("Turning from {} by {} steps", dir, steps);
    return dirs[((dirs.into_iter().position(|&x| x==dir).unwrap() as i32 + steps)%4) as usize];
}
