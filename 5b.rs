fn main() {
    let data = std::fs::read_to_string("5.dat").expect("oops");

    let mut seat: [bool; 916] = [false; 916];

    for line in data.lines() {
        let mut id = 0;
        for ch in line.chars() {
            id = id << 1 | match ch {
                'B' | 'R' => 1,
                _ => 0
            }
        }
        seat[id] = true;
    }

    for (id, taken) in seat.iter().enumerate() {
        if !taken {
            println!("Seat {} is open", id)
        }
    }
}
