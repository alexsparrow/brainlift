use brainlift::interp::{brainfuck_state};
use brainlift::jit::{brainfuck_jit_state, brainfuck_jit};
use brainlift::{stdlib::{MockStdLib, DefaultStdLib}, state::BrainfuckState};

fn compare_interp_jit(code: &str) {
    let mut state1 = BrainfuckState::new();
    let mut stdlib1 = MockStdLib::new("");
    brainfuck_jit_state(code, &mut DefaultStdLib {}, &mut state1);

    let mut state2 = BrainfuckState::new();
    let mut stdlib2 = MockStdLib::new("");
    brainfuck_state(code, &mut DefaultStdLib {}, &mut state2);

    state1.assert_eq(&state2);

    assert_eq!(stdlib1.output, stdlib2.output);
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

#[test]
fn read() {
    let mut stdlib = MockStdLib::new("hello");
    brainfuck_jit_state(",.,.,.,.,.", &mut stdlib, &mut BrainfuckState::new());
    assert_eq!(stdlib.output, "hello".to_string());
}
