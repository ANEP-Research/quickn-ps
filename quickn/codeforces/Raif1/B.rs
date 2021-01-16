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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let s_t: String = scan.token();
        let s: Vec<char> = s_t.chars().collect();
        let (mut d1, mut d2): (usize, usize) = (0, 0);
        for i in 0..n {
            if s[i] == '>' {
                d1 += 1;
            }
            if s[i] == '<' {
                d2 += 1;
            }
        }
        if d1 == 0 || d2 == 0 {
            writeln!(sout, "{}", n).ok();
        } else {
            let mut res = 0;
            let mut c = 0;
            let mut i = 0;
            let mut visited = 0;
            while s[i] == '-' {
                i = (n + i - 1)%n;
            }
            i += 1;
            i %= n;
            while visited <= n {
                if s[i] != '-' {
                    if c > 0 {
                        res += c+1;
                    }
                    c = 0;
                } else {
                    c += 1;
                }
                i += 1;
                i %= n;
                visited += 1;
            }
            writeln!(sout, "{}", res).ok();
        }
    }
}
