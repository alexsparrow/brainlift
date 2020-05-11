#![feature(test)]
use std::env;
use std::fs;

mod interp;
mod jit;
mod state;
mod stdlib;

extern crate test;
use interp::brainfuck_state;
use jit::{brainfuck_jit, brainfuck_jit_compile, brainfuck_jit_run, brainfuck_jit_state};
use state::BrainfuckState;
use test::Bencher;

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
    let mut state1 = BrainfuckState::new();
    brainfuck_jit_state(HELLO_WORLD_SRC, &mut state1);

    let mut state2 = BrainfuckState::new();
    brainfuck_state(HELLO_WORLD_SRC, &mut state2);

    state1.assert_eq(&state2);
}

#[test]
fn nested_loop() {
    interp::brainfuck("+[>[+-]<-]");
}

#[bench]
fn jit_precompile(b: &mut Bencher) {
    let f = brainfuck_jit_compile(HELLO_WORLD_SRC);
    b.iter(|| brainfuck_jit_run(f, &mut BrainfuckState::new()));
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
