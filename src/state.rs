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

    pub fn memory_dmp(&self) {
        for i in 0..1024 {
            if self.mem[i] > 0 {
                println!("{:04}| {}", i, self.mem[i]);
            }
        }
    }

    pub fn assert_eq(&self, other: &Self) {
        assert_eq!(self.pos, other.pos);
        for i in 0..1024 {
            assert!(
                self.mem[i] == other.mem[i],
                "memory at location {} was unequal (left = {}, right = {})",
                i,
                self.mem[i],
                other.mem[i]
            );
        }
    }
}
