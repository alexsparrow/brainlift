fn brainfuck(input: &str) {
    let mut branches: Vec<usize> = Vec::new();
    let mut mem: [u8; 1024] = [0; 1024];
    let mut pos: usize = 0;
    let mut pc = 0;

    while pc < input.len() {
        let c = input.chars().nth(pc).expect("Out of range");
        match c {
            '+' => mem[pos] += 1,
            '-' => mem[pos] -= 1,
            '>' => pos += 1,
            '<' => pos -= 1,
            '[' => {
                branches.push(pc);
                if mem[pos] == 0 {
                    loop {
                        pc += 1;
                        let scan = input.chars().nth(pc).expect("Out of range");

                        if scan == ']' {
                            break;
                        }
                    }
                }
            }
            ']' => {
                if mem[pos] != 0 {
                    pc = *branches.last().expect("Unbalanced delimiter");
                } else {
                    branches.pop().expect("Unbalanced delimiter");
                }
            }
            '.' => print!("{}", mem[pos] as char),
            ',' => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf);
                mem[pos] = buf.chars().nth(0).expect("No input") as u8;
            }
            _ => (),
        }
        pc += 1;
    }
}
