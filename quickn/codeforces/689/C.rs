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
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let mut arr: Vec<usize> = vec![0;n];
        for i in 0..n {
            arr[i] = scan.token();
        }
        let mut pmax: Vec<usize> = vec![0;n+1];
        use std::cmp::max;
        for i in 0..n {
            pmax[i+1] = pmax[i];
            pmax[i+1] = max(pmax[i+1], arr[i]);
        }
        let mut mini = n;
        for _i in 0..n {
            let i = n - _i - 1;
            if arr[i] != i+1 {
                break;
            } else {
                if pmax[i+1] < pmax[i] {
                    break;
                }
            }
            mini -= 1;
        }
        let mut prod: f64 = 1.0;
        for i in 0..m {
            let (r_i, p_i): (usize, f64) = (scan.token(), scan.token());
            if r_i >= mini {
                prod *= 1.0 - p_i;
            }
        }
        if mini == 0 {
            prod = 0.0;
        }
        writeln!(sout, "{}", 1.0 - prod).ok();
    }
}
