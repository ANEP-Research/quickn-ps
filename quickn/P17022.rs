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

use std::collections::BTreeSet;

#[derive(Clone, Debug)]
struct Segtree {
    n: usize,
    data: Vec<usize>,
}

impl Segtree {
    fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0;n*10],
        }
    }

    fn _update(&mut self, node: usize, start: usize, end: usize, idx: usize, val: usize) {
        if !(start > end || idx < start || idx > end) {
            if start == end {
                self.data[node] = val;
            } else {
                let mid = (start+end)/2;
                self._update(node*2, start, mid, idx, val);
                self._update((node*2)+1, mid+1, end, idx, val);
                self.data[node] = self.data[node*2] + self.data[(node*2)+1];
            }
        }
    }

    fn update(&mut self, idx: usize, val: usize) {
        self._update(1, 0, self.n-1, idx, val);
    }

    fn _query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> usize {
        if !(start > end || l > end || r < start) {
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

    fn query(&self, l: usize, r: usize) -> usize {
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
    for i in 0..n {
        arr[i] = scan.token();
        arr[i] -= 1;
    }
    let mut i = n;
    let mut prev = n+1;
    let mut seg = Segtree::new(n);
    while i > 0 {
        if prev > arr[i-1] {
            seg.update(arr[i-1], 1);
        } else {
            break;
        }
        prev = arr[i-1];
        i -= 1;
    }
    let mut res: Vec<usize> = vec![];
    for j in 0..i {
        res.push(seg.query(0, arr[j]) + (i - j) - 1);
        seg.update(arr[j], 1);
    }
    writeln!(sout, "{}", res.len()).ok();
    for a in res {
        write!(sout, "{} ", a).ok();
    }
}
