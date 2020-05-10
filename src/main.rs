#![feature(test)]
use std::env;
use std::fs;
use std::mem;

mod interp;
mod jit;
mod stdlib;

extern crate test;
use test::Bencher;

fn brainfuck_jit_compile(input: &str) -> *const u8 {
    let mut j = jit::JIT::new();
    return j.compile(input).expect("Wat");
}

fn brainfuck_jit_run(f: *const u8) {
    let mut mem: [u8; 1024] = [0; 1024];
    let mut pos: usize = 0;
    let func = unsafe { mem::transmute::<_, fn(&mut [u8; 1024], &mut usize) -> isize>(f) };
    func(&mut mem, &mut pos);
}

fn brainfuck_jit(input: &str) {
    let f = brainfuck_jit_compile(input);
    brainfuck_jit_run(f);
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
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

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

const HELLO_WORLD_SRC: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

#[test]
fn hello_world() {
    brainfuck_jit(HELLO_WORLD_SRC);
}

#[bench]
fn jit_precompile(b: &mut Bencher) {
    let f = brainfuck_jit_compile(HELLO_WORLD_SRC);
    b.iter(|| brainfuck_jit_run(f));
}

#[bench]
fn jit(b: &mut Bencher) {
    b.iter(|| brainfuck_jit(HELLO_WORLD_SRC));
}

#[bench]
fn interp(b: &mut Bencher) {
    b.iter(|| interp::brainfuck(HELLO_WORLD_SRC));
}

// #[test]
// fn read() {
//     brainfuck_jit(",");
// }
