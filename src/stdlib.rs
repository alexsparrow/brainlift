use std::io::stdin;

pub trait StdLib {
    fn getc(&mut self) -> u8;
    fn putc(&mut self, c: u8) -> u8;
}

pub struct DefaultStdLib {}

impl StdLib for DefaultStdLib {
    fn getc(&mut self) -> u8 {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Oops");
        return buf.chars().nth(0).expect("No input") as u8;
    }

    fn putc(&mut self, c: u8) -> u8 {
        print!("{}", c as char);
        c
    }
}

pub struct MockStdLib<'a> {
    input: &'a str,
    input_pos: usize,
    pub output: String,
}

impl<'a> MockStdLib<'a> {
    pub fn new(input: &'a str) -> Self {
        MockStdLib {
            input,
            input_pos: 0,
            output: String::new(),
        }
    }
}

impl StdLib for MockStdLib<'_> {
    fn getc(&mut self) -> u8 {
        let c = self
            .input
            .chars()
            .nth(self.input_pos)
            .expect("Out of range of input") as u8;
        self.input_pos += 1;
        return c;
    }

    fn putc(&mut self, c: u8) -> u8 {
        self.output.push(c as char);
        c
    }
}
