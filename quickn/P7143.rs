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

const INF: i32 = 1_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let v: i32 = 35;
    writeln!(sout, "{}", v).ok();
    let start = 0;
    let end = v-1;
    let mut ans = 1;
    for i in 0..(v-1) {
        if i % 2 == 0 {
            ans += 5;
            writeln!(sout, "{} {} {} {} {}", 2, i+1, 1 << ((v/2)-(i/2)+1), i+2, 1 << ((v/2)-(i/2)-1)).ok();
        } else {
            ans += 3;
            writeln!(sout, "{} {} {}", 1, i+1, -(1 << ((v/2)-(i/2)+1))).ok();
        }
    }
    ans += 12;
    writeln!(sout, "0").ok();
    writeln!(sout, "3").ok();
    for i in 0..3 {
        writeln!(sout, "{} {}", start, end).ok();
    }
    //writeln!(sout, "{}", ans).ok();
}