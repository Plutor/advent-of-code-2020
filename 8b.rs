fn main() {
    let data = std::fs::read_to_string("8.dat").expect("oops");

    let mut instructions = data.lines().collect::<Vec<&str>>();

    struct State {
        acc: i32,
        pc: i32,
        ch: bool,
    };
    let mut states = Vec::new();
    states.push(State{acc:0, pc:0, ch:false});

    loop {
        let state = states.pop().expect("oops");
        let acc = state.acc;
        let pc = state.pc;
        let ch = state.ch;
        if pc as usize >= instructions.len() {
            println!("pc={}, acc={}, done!", pc, acc);
            break;
        }
        let instruction = instructions[pc as usize];
        println!("pc={}, acc={}, running {}", pc, acc, instruction);
        let mut iter = instruction.split_whitespace();
        let itype = iter.next().unwrap();
        let ival: i32 = iter.next().unwrap().parse().unwrap();
        match itype {
            "acc" => { states.insert(0, State{acc: acc + ival, pc: pc+1, ch:ch}) },
            "jmp"=> { states.insert(0, State{acc: acc, pc: pc+ival, ch:ch});
                      if !ch { states.insert(0, State{acc: acc, pc: pc+1, ch:true}) } },
            _ => { if !ch { states.insert(0, State{acc: acc, pc: pc+ival, ch:true}); }
                   states.insert(0, State{acc: acc, pc: pc+1, ch:ch}) },
        }
    }
}

