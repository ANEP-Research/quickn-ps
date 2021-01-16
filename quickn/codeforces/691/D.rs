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

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    cost: f64,
    idx: usize,
}

impl Vertex {
    fn new(cost: f64, idx: usize) -> Self {
        Self { cost, idx }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.cost < other.cost {
            Ordering::Less
        } else if self.cost > other.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost < other.cost {
            Ordering::Less
        } else if self.cost > other.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.idx == other.idx
    }
}

impl Eq for Vertex {}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<(i64, i64)> = vec![(0, 0); n];
    for i in 0..n {
        arr[i] = (scan.token(), scan.token());
    }
    arr.sort_by(|&(_, a1), &(_, a2)| a1.cmp(&a2));
    //arr.reverse();
    use std::collections::BinaryHeap;
    for k in 0..n {
        let mut res: f64 = 0.0;
        {
            let mut arr2: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
            for i in 0..n {
                arr2[i] = (arr[i].0 as f64, arr[i].1 as f64);
            }
            let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();
            for i in (n - k - 1)..n {
                heap.push(Vertex::new(arr2[i].0 - arr2[i].1, i));
            }
            let mut i = n - k - 1;
            while i > 0 {
                while let Some(v) = heap.peek() {
                    let d = v.cost;
                    let idx = v.idx;
                    if arr2[i - 1].1 >= d * 2.0 {
                        arr2[i - 1].1 -= d * 2.0;
                        arr2[idx].1 = arr2[idx].0;
                        heap.pop();
                    } else {
                        let x = arr2[i - 1].1;
                        arr2[i - 1].1 = 0.0;
                        arr2[idx].1 += x / 2.0;
                        heap.pop();
                        heap.push(Vertex::new(arr2[idx].0 - arr2[idx].1, idx));
                        break;
                    }
                }
                arr2[i - 1].1 = 0.0;
                i -= 1;
            }
            let mut sum: f64 = 0.0;
            for i in 0..n {
                sum += arr2[i].1;
            }
            res = sum;
        }
        {
            let mut arr2: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
            for i in 0..n {
                arr2[i] = (arr[i].0 as f64, arr[i].1 as f64);
            }
            arr2.reverse();
            let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();
            for i in (n - k - 1)..n {
                heap.push(Vertex::new(arr2[i].0 - arr2[i].1, i));
            }
            let mut i = n - k - 1;
            while i > 0 {
                while let Some(v) = heap.peek() {
                    let d = v.cost;
                    let idx = v.idx;
                    if arr2[i - 1].1 >= d * 2.0 {
                        arr2[i - 1].1 -= d * 2.0;
                        arr2[idx].1 = arr2[idx].0;
                        heap.pop();
                    } else {
                        let x = arr2[i - 1].1;
                        arr2[i - 1].1 = 0.0;
                        arr2[idx].1 += x / 2.0;
                        heap.pop();
                        heap.push(Vertex::new(arr2[idx].0 - arr2[idx].1, idx));
                        break;
                    }
                }
                arr2[i - 1].1 = 0.0;
                i -= 1;
            }
            let mut sum: f64 = 0.0;
            for i in 0..n {
                sum += arr2[i].1;
            }
            if res < sum {
                res = sum;
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
