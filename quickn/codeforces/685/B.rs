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
        let (n, q): (usize, usize) = (scan.token(), scan.token());
        let s: String = scan.token();
        let arr: Vec<char> = s.chars().collect();
        for _query in 0..q {
            let (mut l_i, mut r_i): (usize, usize) = (scan.token(), scan.token());
            l_i -= 1;
            r_i -= 1;
            let mut res = false;
            for i in l_i..r_i {
                let mut j = 0;
                let mut pos = l_i;
                let mut next = false;
                let mut prev = std::usize::MAX;
                while j < n {
                    if arr[pos] == arr[j] {
                        if next {
                            if prev != j-1 {
                                next = false;
                                prev = j;
                                pos += 1;
                            }
                        } else {
                            if pos == i {
                                next = true;
                            }
                            prev = j;
                            pos += 1;
                        }
                    }
                    if pos == r_i + 1 {
                        res |= true;
                        break;
                    }
                    j += 1;
                }
            }
            if res {
                writeln!(sout, "YES").ok();
            } else {
                writeln!(sout, "NO").ok();
            }
        }
    }
}
