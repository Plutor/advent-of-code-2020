#[derive(PartialEq, Eq, Clone)]
enum S {
    FLOOR = 0,
    EMPTY,
    FULL,
}

fn main() {
    let data = std::fs::read_to_string("11.dat").expect("oops");
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let rowlen = data.lines().next().unwrap().len();
    let mut grid = Vec::new();
    for l in data.lines() {
        for c in l.chars() {
            grid.push(match c {
                '.' => S::FLOOR,
                _ => S::EMPTY,
            });
        }
    }
    println!("Loaded {}x{} grid", rowlen, grid.len()/rowlen);

    let mut lastgrid = Vec::new();
    let mut fullcount = 0;
    loop {
        fullcount = 0;
        lastgrid = grid.clone();
        // Update grid with lastgrid.
        for n in 0..grid.len() {
            let x = (n / rowlen) as i32;
            let y = (n % rowlen) as i32;
            let mut ncount = 0;
            for dir in directions.iter() {
                ncount += seesfull(&lastgrid, rowlen, x, y, *dir);
            }
            if lastgrid[n] == S::EMPTY && ncount == 0 {
                grid[n] = S::FULL;
            } else if lastgrid[n] == S::FULL && ncount >= 5 {
                grid[n] = S::EMPTY;
            }
            if grid[n] == S::FULL {
                fullcount += 1;
            }
        }
        if lastgrid == grid {
            println!("Done, no changes, fullcount={}", fullcount);
            break;
        } else {
            println!("fullcount={}", fullcount);
        }
    }
}

// Returns 1/0 instead of bool so adding is easy.
fn seesfull(grid: &Vec<S>, rowlen: usize, mut x: i32, mut y: i32, (xdir, ydir): (i32, i32)) -> usize {
    loop {
        x += xdir;
        y += ydir;
        if y < 0 || y >= rowlen as i32 { return 0; }
        if x < 0 || x >= (grid.len()/rowlen) as i32 { return 0; }
        match grid.get((x as usize)*rowlen+(y as usize)) {
            Some(S::FULL) => return 1,
            Some(S::EMPTY) => return 0,
            _ => {}
        };
    }
}
