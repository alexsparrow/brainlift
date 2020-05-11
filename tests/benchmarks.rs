#![feature(test)]
extern crate test;
use test::Bencher;
use brainlift::jit::{brainfuck_jit, brainfuck_jit_compile, brainfuck_jit_run};
use brainlift::state::BrainfuckState;
use brainlift::interp::{brainfuck};

const HELLO_WORLD_SRC: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

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
    b.iter(|| brainfuck(HELLO_WORLD_SRC));
}

