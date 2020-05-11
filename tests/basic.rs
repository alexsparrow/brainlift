use brainlift::interp::{brainfuck_state};
use brainlift::jit::{brainfuck_jit_state, brainfuck_jit};
use brainlift::{stdlib::{MockStdLib, DefaultStdLib}, state::BrainfuckState};

fn compare_interp_jit(code: &str) -> (brainlift::state::BrainfuckState, brainlift::stdlib::MockStdLib<'_>) {
    let mut state1 = BrainfuckState::new();
    let mut stdlib1 = MockStdLib::new("hello");
    brainfuck_jit_state(code, &mut stdlib1, &mut state1);

    let mut state2 = BrainfuckState::new();
    let mut stdlib2 = MockStdLib::new("hello");
    brainfuck_state(code, &mut stdlib2, &mut state2);

    state1.assert_eq(&state2);

    assert_eq!(stdlib1.output, stdlib2.output);

    return (state1, stdlib1);
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
    let (_, stdlib) = compare_interp_jit( ",.,.,.,.,.");
    assert_eq!(stdlib.output, "hello".to_string());
}
