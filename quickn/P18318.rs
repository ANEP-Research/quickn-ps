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

struct SegmentTree {
    n: usize,
    data: Vec<i32>,
    lazy: Vec<i32>,
}

use std::cmp::min;

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut res = Self {
            n: n,
            data: vec![std::i32::MAX;n<<5],
            lazy: vec![0;n<<5],
        };
    }

    fn _add_update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, x: i32) {
        if start <= end && !(start > r || end < l) {
            if l <= start && end <= r {
                self.data[node] += x;
                if start != end {
                    self.lazy[node*2 + 1] += x;
                    self.lazy[node*2] += x;
                }
            } else {
                let mid = (start+end)/2;
                self.add_update(node*2, start, mid, l, r, x);
                self.add_update(node*2 + 1, mid+1, end, l, r, x);
                self.data[node] = min(self.data[node*2], self.data[node*2 + 1]);
            }
        }
    }

    fn add_update(&mut self, l: usize, r: usize, x: i32) {
        self._add_update(1, 0, self.n-1, l, r, x);
    }

    fn _update(&mut self, node: usize, start: usize, end: usize, idx: usize, x: i32) {
        if start <= end && start <= idx && idx <= end {
            if start == end {
                self.data[node] = x;
            } else {
                let mid = (start+end)/2;
                self.update(node*2, start, mid, idx, x);
                self.update(node*2 + 1, mid+1, end, idx, x);
                self.data[node] = min(self.data[node*2], self.data[node*2 + 1]);
            }
        }
    }

    fn update(&mut self, idx: usize, x: i32) {
        self._update(1, 0, self.n-1, idx, x);
    }

    fn _query(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if start <= end && !(start > r || end < l) {
            self.data[node] += self.lazy[node];
            if start != end {
                self.lazy[node*2 + 1] += self.lazy[node];
                self.lazy[node*2] += self.lazy[node];
            }
            self.lazy[node] = 0;
            if l <= start && end <= r {
                self.data[node]
            } else {
                let mid = (start+end)/2;
                min(self.query(node*2, start, mid, l, r), self.query(node*2 + 1, mid+1, end, l, r))
            }
        }
    }

    fn query(&mut self, l: usize, r: usize) -> i32 {
        self._query(1, 0, self.n-1, l, r)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, p): (i32, usize) = (scan.token(), scan.token());
    let mut arr: Vec<(i32, i32)> = vec![(0, 0)];
    let mut adj: Vec<Vec<usize>> = vec![vec![];p*2 + 2];
    for i in 0..p {
        let (x1, y1, x2, y2): (i32, i32, i32, i32) = (scan.token(), scan.token(), scan.token(), scan.token());
        arr.push((x1, y1));
        arr.push((x2, y2));
        
    }
}
