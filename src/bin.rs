use std::env;
use std::fs;

use brainlift::jit::brainfuck_jit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).expect("Supply brainfuck file");

    println!("{}", fname);
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    brainfuck_jit(&contents);
}
