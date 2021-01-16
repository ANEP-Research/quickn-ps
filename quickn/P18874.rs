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

struct SegTree {
    n: usize,
    data: Vec<i64>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0;n*10],
        }
    }

    fn _update(&mut self, node: usize, start: usize, end: usize, pos: usize) {
        if !(end < pos || pos < start || start > end) {
            if start == end {
                self.data[node] += 1;
            } else {
                let mid = (start+end)/2;
                self._update(node*2, start, mid, pos);
                self._update((node*2)+1, mid+1, end, pos);
                self.data[node] = self.data[node*2] + self.data[(node*2)+1];
            }
        }
    }

    fn update(&mut self, pos: usize) {
        self._update(1, 0, self.n-1, pos);
    }

    fn _query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if !(end < l || r < start || start > end) {
            if l <= start && end <= r {
                self.data[node]
            } else {
                let mid = (start+end)/2;
                self._query(node*2, start, mid, l, r) + self._query((node*2)+1, mid+1, end, l, r)
            }
        } else {
            0
        }
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self._query(1, 0, self.n-1, l, r)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<usize> = vec![0;n];
    let mut query: Vec<i64> = vec![0;n+1];
    let mut seg = SegTree::new(n+2);
    for i in 0..n {
        arr[i] = scan.token();
        query[arr[i]] += seg.query(arr[i]+1, n);
        seg.update(arr[i]);
    }
    let mut res = 0;
    for i in 0..n {
        writeln!(sout, "{}", res).ok();
        res += query[i];
    }
}
