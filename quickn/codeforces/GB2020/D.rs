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

fn s2vec(s: String) -> Vec<char> {
    s.chars().collect()
}

use std::collections::BinaryHeap;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let mut w: Vec<i64> = vec![0;n];
        let mut res = 0;
        for i in 0..n {
            w[i] = scan.token();
            res += w[i];
        }
        let mut deg: Vec<usize> = vec![0;n];
        for i in 0..(n-1) {
            let (u, v): (usize, usize) = (scan.token(), scan.token());
            deg[u-1] += 1;
            deg[v-1] += 1;
        }
        let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        for i in 0..n {
            if deg[i] >= 2 {
                heap.push((w[i], i));
            }
        }
        for i in 0..(n-1) {
            write!(sout, "{} ", res).ok();
            if let Some((w, i)) = heap.pop() {
                res += w;
                deg[i] -= 1;
                if deg[i] >= 2 {
                    heap.push((w, i));
                }
            }
        }
        writeln!(sout, "").ok();
    }
}
