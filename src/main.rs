use std::env;
use std::fs;
use std::mem;

mod jit;
mod interp;

fn brainfuck_jit(input: &str) {
    let mut mem: [u8; 1024] = [0; 1024];
    let mut pos: usize = 0;
    let mut j = jit::JIT::new();
    let f= j.compile(input).expect("Wat");
    let func = unsafe { mem::transmute::<_, fn(&mut [u8; 1024], &mut usize) -> isize>(f) };
    func(&mut mem, &mut pos);
    memory_dmp(&mem);
}

fn memory_dmp(mem: &[u8; 1024]) {
    for i in 0..1024 {
        if mem[i] > 0 {
            println!("{:04}| {}", i, mem[i]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).expect("Supply brainfuck file");

    
    println!("{}", fname);
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");

    brainfuck_jit(&contents);
}

#[test]
fn basic() {
    brainfuck_jit("+>");
}

#[test]
fn wile() {
    brainfuck_jit(">+++++[<+>-].");
}

#[test]
fn hello_world() {
    brainfuck_jit("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
}

#[test]
fn read() {
    brainfuck_jit(",");
}
