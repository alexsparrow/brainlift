pub struct BrainfuckState {
    pub mem: [u8; 1024],
    pub pos: usize,
}

impl BrainfuckState {
    pub fn new() -> Self {
        return BrainfuckState {
            mem: [0; 1024],
            pos: 0,
        };
    }
}
