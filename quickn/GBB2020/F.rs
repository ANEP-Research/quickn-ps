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

struct SegmentTree {
    n: usize,
    data: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0;n*16],
        }
    }

    fn _update(&mut self, node: usize, start: usize, end: usize, idx: usize, x: i64) {
        if !(start > end || start > idx || end < idx) {
            if start == end {
                self.data[node] = x;
            } else {
                let mid = (start+end)/2;
                self._update(node*2, start, mid, idx, x);
                self._update((node*2)+1, mid+1, end, idx, x);
                self.data[node] = max(self.data[node*2], self.data[(node*2)+1]);
            }
        }
    }

    fn _query(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if start > end || end < l || r < start {
            0
        } else {
            if l <= start && end <= r {
                self.data[node]
            } else {
                let mid = (start+end)/2;
                max(self._query(node*2, start, mid, l, r), self._query((node*2)+1, mid+1, end, l, r))
            }
        }
    }

    fn update(&mut self, idx: usize, x: i64) {
        self._update(1, 0, self.n-1, idx, x);
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self._query(1, 0, self.n-1, l, r)
    }
}

use std::collections::BinaryHeap;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<i64> = vec![0;n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    arr.sort();
    arr.reverse();
    let (mut i, mut j) = (0, 1);
    let mut s1: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    let mut s2 = SegmentTree::new(n);
    let mut arr2: Vec<i64> = vec![0;n-4];
    for i in 2..(n-4) {
        arr2[i] = -arr[i];
        for j in 1..5 {
            arr2[i] += arr[i+j];
        }
        s1.push((arr2[i], i));
    }
    let mut res = -1;
    while j < n-5 {
        while j < n-5 && arr[j] + arr[j+1] > arr[i] {
            while let Some(&(x, idx)) = s1.peek() {
                if x > arr[j] {
                    s1.pop();
                    s2.update(idx, x + arr[idx]*2);
                } else {
                    break;
                }
            }
            let (mut l, mut r) = (j+1, n-1);
            while l < r {
                let mid = (l+r)/2;
                if arr[mid] + arr[j] <= arr[i] {
                    r = mid;
                } else {
                    l = mid+1;
                }
            }
            if j < r-1 {
                let q = s2.query(j+1, r-1);
                if q != 0 {
                    res = max(res, q + arr[j] + arr[i]);
                }
            }
            j += 1;
        }
        i += 1;
        if i == j {
            j += 1;
        }
    }
    writeln!(sout, "{}", res).ok();
}
