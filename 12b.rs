fn main() {
    let data = std::fs::read_to_string("12.dat").expect("oops");

    let (mut wx, mut wy): (i32, i32) = (10, -1);
    let (mut x, mut y) = (0,0);
    for line in data.lines() {
        let mut chars = line.chars();
        let instr = chars.next().unwrap();
        let dist: i32 = chars.fold(String::new(), |a,c|{a+&c.to_string()}).parse().unwrap();

        match instr {
            'N' => wy -= dist,
            'S' => wy += dist,
            'E' => wx += dist,
            'W' => wx -= dist,
            'R' => {let (nwx, nwy) = turnby(wx, wy, dist/90); wx=nwx; wy=nwy},
            'L' => {let (nwx, nwy) = turnby(wx, wy, 4-dist/90); wx=nwx; wy=nwy},
            'F' => {x += wx*dist; y += wy*dist }
            _ => panic!(),
        }
        println!("Move {} by {}, waypoint=({},{}), boat=({},{})", instr, dist, wx, wy, x, y);
    }
}

fn turnby(wx: i32, wy: i32, steps: i32) -> (i32, i32) {
    return match steps {
        1 => (wy, -wx),
        2 => (-wx, -wy),
        3 => (-wy, wx),
        _ => (wx, wy),
    }
}
