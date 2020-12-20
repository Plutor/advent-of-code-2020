fn main() {
    let data = std::fs::read_to_string("8.dat").expect("oops");

    let mut instructions = data.lines().collect::<Vec<&str>>();


    let mut acc: i32 = 0;
    let mut pc: i32 = 0;
    loop {
        let instruction = instructions[pc as usize];
        println!("pc={}, acc={}, running {}", pc, acc, instruction);
        if instruction == "!!!" {
            break;
        }
        let mut iter = instruction.split_whitespace();
        let itype = iter.next().unwrap();
        let ival: i32 = iter.next().unwrap().parse().unwrap();
        instructions[pc as usize] = "!!!";
        pc = match itype {
            "acc" => { acc += ival; pc+1 },
            "jmp" => { pc+ival },
            _ => pc+1
        }
    }
}

