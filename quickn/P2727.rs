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

use std::collections::BTreeSet;

struct Run {
    res: Vec<(usize, usize)>,
}

impl Run {
    fn new() -> Self {
        Self {
            res: vec![],
        }
    }
    fn run(&mut self, n: u64, off: usize) {
        if n == 0 {
            return;
        } else if n == 1 {
            self.res.push((off, 0));
        } else if n % 2 == 0 {
            self.run(n/2, off+1);
        } else {
            let (mut s3, mut s3_idx) = (1, 0);
            while s3 <= n {
                s3 *= 3;
                s3_idx += 1;
            }
            s3 /= 3;
            s3_idx -= 1;
            self.res.push((off, s3_idx));
            self.run(n-s3, off);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let mut t: usize = scan.token();
    for _i in 0..t {
        let mut n: u64 = scan.token();
        let mut res: Vec<(usize, usize)> = vec![];
        let mut run = Run::new();
        run.run(n, 0);
        writeln!(sout, "{}", run.res.len()).ok();
        for (a, b) in run.res {
            writeln!(sout, "{} {}", a, b).ok();
        }
    }
}
