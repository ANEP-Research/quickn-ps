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

const BITS: usize = 32;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<i32> = vec![0;n+1];
    let mut pxor: Vec<(i32, usize)> = vec![(0, 0);n+1];
    for i in 1..=n {
        pxor[i] = pxor[i-1];
        pxor[i].1 = i;
        arr[i] = scan.token();
        pxor[i].0 ^= arr[i];
    }
    pxor.sort();
    use std::cmp::{min, max};
    let mut res = std::usize::MAX;
    for i in 0..n {
        let x = pxor[i].0 ^ pxor[i+1].0;
        let mm = min(pxor[i].1, pxor[i+1].1);
        let ma = max(pxor[i].1, pxor[i+1].1);
        if ma < n {
            if x > arr[ma+1] {
                res = min(res, ma - mm);
            }
        }
        if mm > 1 {
            if x < arr[mm-1] {
                res = min(res, ma - mm);
            }
        }
    }
    pxor.reverse();
    for i in 0..n {
        let x = pxor[i].0 ^ pxor[i+1].0;
        let mm = min(pxor[i].1, pxor[i+1].1);
        let ma = max(pxor[i].1, pxor[i+1].1);
        if ma < n {
            if x > arr[ma+1] {
                res = min(res, ma - mm);
            }
        }
        if mm > 1 {
            if x < arr[mm-1] {
                res = min(res, ma - mm);
            }
        }
    }
    writeln!(sout, "{}", res).ok();
}
