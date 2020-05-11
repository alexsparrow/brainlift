use brainlift::interp::{brainfuck_state};
use brainlift::jit::{brainfuck_jit_state};
use brainlift::state::BrainfuckState;

fn compare_interp_jit(code: &str) {
    let mut state1 = BrainfuckState::new();
    brainfuck_jit_state(code, &mut state1);

    let mut state2 = BrainfuckState::new();
    brainfuck_state(code, &mut state2);

    state1.assert_eq(&state2);
}

#[test]
fn basic() {
    compare_interp_jit("+>");
}

#[test]
fn wile() {
    compare_interp_jit(">+++++[<+>-].");
}


const HELLO_WORLD_SRC: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

#[test]
fn hello_world() {
    compare_interp_jit(HELLO_WORLD_SRC);
}

#[test]
fn nested_loop() {
    compare_interp_jit("+[>[+-]<-]");
}

// #[test]
// fn read() {
//     brainfuck_jit(",");
// }
