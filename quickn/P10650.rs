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

struct Segtree {
    n: usize,
    data: Vec<i32>,
    f: Box<Fn(i32, i32) -> i32>,
    def: i32,
}

impl Segtree {
    fn new(n: usize, f: Box<Fn(i32, i32) -> i32>, def: i32) -> Self {
        Self {
            n,
            data: vec![def;n*17],
            f,
            def,
        }
    }

    fn update(&mut self, target: usize, x: i32) {
        self._update(1, 0, self.n-1, target, x);
    }

    fn _update(&mut self, node: usize, l: usize, r: usize, target: usize, x: i32) {
        if !(l > target || target > r) {
            if l == r {
                self.data[node] = x;
            } else {
                let mid = (l+r)/2;
                self._update(node*2, l, mid, target, x);
                self._update((node*2)+1, mid+1, r, target, x);
                self.data[node] = (self.f)(self.data[node*2], self.data[(node*2)+1]);
            }
        }
    }

    fn query(&self, start: usize, end: usize) -> i32 {
        self._query(1, 0, self.n-1, start, end)
    }

    fn _query(&self, node: usize, l: usize, r: usize, start: usize, end: usize) -> i32 {
        if l > end || start > r {
            self.def
        } else {
            if start <= l && r <= end {
                self.data[node]
            } else {
                let mid = (l+r)/2;
                (self.f)(self._query(node*2, l, mid, start, end), self._query((node*2)+1, mid+1, r, start, end))
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn metric(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, q): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<Point> = vec![];
    for i in 0..n {
        let (x, y): (i32, i32) = (scan.token(), scan.token());
        arr.push(Point::new(x, y));
    }
    let mut sumseg = Segtree::new(n-1, Box::new(|x: i32, y: i32| x + y), 0);
    for i in 0..(n-1) {
        sumseg.update(i, arr[i].metric(arr[i+1]));
        //dbg!(sumseg.query(i, i));
    }
    if n <= 3 {
        for _case in 0..q {
            let kind: char = scan.token();
            if kind == 'Q' {
                let (mut start, mut end): (usize, usize) = (scan.token(), scan.token());
                start -= 1;
                end -= 1;
                if end - start == 0 {
                    writeln!(sout, "0").ok();
                } else if end - start == 1 {
                    writeln!(sout, "{}", arr[start].metric(arr[end])).ok();
                } else {
                    let mut d = 0;
                    for i in start..(end-1) {
                        let d2 = (arr[i].metric(arr[i+1]) + arr[i+1].metric(arr[i+2])) - arr[i].metric(arr[i+2]);
                        d = max(d2, d);
                    }
                    writeln!(sout, "{}", sumseg.query(start, end-1) - d).ok();
                }
            } else { 
                let mut idx: usize = scan.token();
                idx -= 1;
                let (x, y): (i32, i32) = (scan.token(), scan.token());
                arr[idx] = Point::new(x, y);
                if idx < n-1 {
                    sumseg.update(idx, arr[idx].metric(arr[idx+1]));
                }
                if idx > 0 {
                    sumseg.update(idx-1, arr[idx].metric(arr[idx-1]));
                }
            }
        }
    } else {
        let mut maxseg = Segtree::new(n-2, Box::new(|x: i32, y: i32| max(x, y)), 0);
        for i in 0..(n-2) {
            maxseg.update(i, (arr[i].metric(arr[i+1]) + arr[i+1].metric(arr[i+2])) - arr[i].metric(arr[i+2]));
        }
        for _case in 0..q {
            let kind: char = scan.token();
            if kind == 'Q' {
                let (mut start, mut end): (usize, usize) = (scan.token(), scan.token());
                start -= 1;
                end -= 1;
                if end - start == 0 {
                    writeln!(sout, "0").ok();
                } else if end - start == 1 {
                    writeln!(sout, "{}", arr[start].metric(arr[end])).ok();
                } else {
                    writeln!(sout, "{}", sumseg.query(start, end-1) - maxseg.query(start, end-2)).ok();
                }
            } else { 
                let mut idx: usize = scan.token();
                idx -= 1;
                let (x, y): (i32, i32) = (scan.token(), scan.token());
                arr[idx] = Point::new(x, y);
                if idx < n-1 {
                    sumseg.update(idx, arr[idx].metric(arr[idx+1]));
                }
                if idx > 0 {
                    sumseg.update(idx-1, arr[idx].metric(arr[idx-1]));
                    if idx < n-2 {
                        maxseg.update(idx-1, (arr[idx-1].metric(arr[idx]) + arr[idx].metric(arr[idx+1])) - arr[idx-1].metric(arr[idx+1]));
                    }
                }
                if idx < n-3 {
                    maxseg.update(idx, (arr[idx].metric(arr[idx+1]) + arr[idx+1].metric(arr[idx+2])) - arr[idx].metric(arr[idx+2]));
                }
                if idx > 1 {
                    maxseg.update(idx-2, (arr[idx-2].metric(arr[idx-1]) + arr[idx-1].metric(arr[idx])) - arr[idx-2].metric(arr[idx]));
                }
            }
        }
    }
}
