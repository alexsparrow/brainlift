use std::io::stdin;

pub fn putc(a: u8) -> u8 {
    print!("{}", a as char);
    a
}

pub fn getc() -> u8 {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Oops");
    return buf.chars().nth(0).expect("No input") as u8;
}
