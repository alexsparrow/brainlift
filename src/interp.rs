use crate::{
    state::BrainfuckState,
    stdlib::{getc, putc},
};

pub fn brainfuck(input: &str) {
    brainfuck_state(input, &mut BrainfuckState::new());
}

pub fn brainfuck_state(input: &str, state: &mut BrainfuckState) {
    let mut branches: Vec<usize> = Vec::new();
    let mut pc = 0;

    while pc < input.len() {
        let c = input.chars().nth(pc).expect("Out of range");
        match c {
            '+' => state.mem[state.pos] += 1,
            '-' => state.mem[state.pos] -= 1,
            '>' => state.pos += 1,
            '<' => state.pos -= 1,
            '[' => {
                branches.push(pc);
                if state.mem[state.pos] == 0 {
                    let mut stack_depth = 1;
                    loop {
                        pc += 1;
                        match input.chars().nth(pc).expect("Out of range") {
                            '[' => stack_depth += 1,
                            ']' => {
                                stack_depth -= 1;
                                if stack_depth == 0 {
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            ']' => {
                if state.mem[state.pos] != 0 {
                    pc = *branches.last().expect("Unbalanced delimiter");
                } else {
                    branches.pop().expect("Unbalanced delimiter");
                }
            }
            '.' => {
                putc(state.mem[state.pos]);
            }
            ',' => {
                state.mem[state.pos] = getc();
            }
            _ => (),
        }
        pc += 1;
    }
}
