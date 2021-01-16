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

use std::cmp::{min, max};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let mut arr: Vec<i32> = vec![0;n];
        for i in 0..n {
            arr[i] = scan.token();
        }
        let mut l = 0;
        let mut prev = 1;
        let mut sz1 = 0;
        let mut sz2 = 0;
        for i in 0..n {
            for k in prev..arr[i] {
                l += 1;
            }
            if l == 0 {
                sz1 += 1;
            } else {
                l -= 1;
            }
            prev = arr[i]+1;
        }
        l = 0;
        prev = 2*(n as i32);
        arr.reverse();
        for i in 0..n {
            let mut k = prev;
            while k > arr[i] {
                l += 1;
                k -= 1;
            }
            if l == 0 {
                sz2 += 1;
            } else {
                l -= 1;
            }
            prev = arr[i]-1;
        }
        writeln!(sout, "{}", ((n as i32) - sz2 - sz1 + 1).abs()).ok();
    }
}
