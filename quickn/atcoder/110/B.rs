/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
    /* https://github.com/EbTech/rust-algorithms */

    /// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
    /// **REQUIRES** Rust 1.34 or higher
    pub struct UnsafeScanner<R> {
        reader: R,
        buf_str: Vec<u8>,
        buf_iter: str::SplitAsciiWhitespace<'static>,
    }

    impl<R: io::BufRead> UnsafeScanner<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf_str: Vec::new(),
                buf_iter: "".split_ascii_whitespace(),
            }
        }

        /// This function should be marked unsafe, but noone has time for that in a
        /// programming contest. Use at your own risk!
        pub fn token<T: str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buf_iter.next() {
                    return token.parse().ok().expect("Failed parse");
                }
                self.buf_str.clear();
                self.reader
                    .read_until(b'\n', &mut self.buf_str)
                    .expect("Failed read");
                self.buf_iter = unsafe {
                    let slice = str::from_utf8_unchecked(&self.buf_str);
                    std::mem::transmute(slice.split_ascii_whitespace())
                }
            }
        }
    }
}

const MAX: i64 = 10_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let s: String = scan.token();
    let mut i = 0;
    let mut c: Vec<&str> = vec![];
    while i < n {
        if i + 3 >= n {
            c.push(s.get(i..).unwrap());
        } else {
            c.push(s.get(i..(i+3)).unwrap());
        }
        i += 3;
    }
    if c.len() > 1 {
        let mut failed = false;
        let mut p = c[0];
        for i in 0..(c.len()-1) {
            if p != c[i] {
                failed = true;
                break;
            }
        }
        let p_e: Vec<char> = c[c.len()-1].chars().collect();
        let p_e2: Vec<char> = p.chars().collect();
        for i in 0..p_e.len() {
            if p_e[i] != p_e2[i] {
                failed = true;
                break;
            }
        }
        let t = "110110110110";
        if let None = t.find(p) {
            failed = true;
        }
        let mut res: i64 = MAX;
        if failed {
            writeln!(sout, "0").ok();
        } else {
            if p == "101" || p == "011" {
                if n % 3 == 0 {
                    res -= 1;
                }
            }
            writeln!(sout, "{}", res - (c.len() as i64) + 1).ok();
        }
    } else {
        let a = c[0];
        if a == "1" {
            writeln!(sout, "{}", 2*MAX).ok();
        } else if a == "0" {
            writeln!(sout, "{}", MAX).ok();
        } else if a == "11" || a == "10" {
            writeln!(sout, "{}", MAX).ok();
        } else if a == "01" {
            writeln!(sout, "{}", MAX-1).ok();
        } else if a == "101" || a == "011" {
            writeln!(sout, "{}", MAX-1).ok();
        } else if a == "110" {
            writeln!(sout, "{}", MAX).ok();
        } else {
            writeln!(sout, "{}", 0).ok();
        }
    }
}
