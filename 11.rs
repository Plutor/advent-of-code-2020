#[derive(PartialEq, Eq, Clone)]
enum S {
    FLOOR = 0,
    EMPTY,
    FULL,
}

fn main() {
    let data = std::fs::read_to_string("11.dat").expect("oops");

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
            let mut ncount = isfull(&lastgrid, rowlen, x-1, y-1) +
                             isfull(&lastgrid, rowlen, x-1, y) +
                             isfull(&lastgrid, rowlen, x-1, y+1) +
                             isfull(&lastgrid, rowlen, x, y-1) +
                             isfull(&lastgrid, rowlen, x, y+1) +
                             isfull(&lastgrid, rowlen, x+1, y-1) +
                             isfull(&lastgrid, rowlen, x+1, y) +
                             isfull(&lastgrid, rowlen, x+1, y+1);
            if lastgrid[n] == S::EMPTY && ncount == 0 {
                grid[n] = S::FULL;
            } else if lastgrid[n] == S::FULL && ncount >= 4 {
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
fn isfull(grid: &Vec<S>, rowlen: usize, x: i32, y: i32) -> usize {
    if y < 0 || y > (rowlen-1) as i32 { return 0; }
    if x < 0 { return 0; }
    return match grid.get((x as usize)*rowlen+(y as usize)) {
        Some(S::FULL) => 1,
        _ => 0
    };
}
