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

#[derive(Clone, Copy, Debug)]
struct Segment {
    idx: usize,
    t: f64,
    left_endpoint: bool,
}

impl Segment {
    fn new(idx: usize, t: f64, left_endpoint: bool) -> Self {
        Self {
            idx,
            t,
            left_endpoint
        }
    }
}

use std::cmp::Ordering;

fn f64cmp(a: f64, b: f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let r: i64 = scan.token();
    let (x, y, vx, vy): (i64, i64, i64, i64) = (scan.token(), scan.token(), scan.token(), scan.token());
    let mut res = 0;
    let mut seg: Vec<(f64, f64)> = vec![];
    for i in 0..n {
        let (xi, yi, vxi, vyi): (i64, i64, i64, i64) = (scan.token(), scan.token(), scan.token(), scan.token());
        let dx = x - xi;
        let dy = y - yi;
        let dvx = vx - vxi;
        let dvy = vy - vyi;
        let a = dvx.pow(2) + dvy.pow(2);
        let bd2 = dx*dvx + dy*dvy;
        let c = dx.pow(2) + dy.pow(2) - r.pow(2);
        if a == 0 {
            if c <= 0 {
                res += 1;
            }
        } else {
            let dd4 = bd2.pow(2) - a*c;
            if dd4 >= 0 {
                let sqrtdd4 = (dd4 as f64).sqrt();
                let mut start = (-(bd2 as f64) - sqrtdd4)/(a as f64);
                let end = (-(bd2 as f64) + sqrtdd4)/(a as f64);
                if start < 0.0 {
                    start = 0.0;
                }
                if start <= end {
                    seg.push((start, end));
                }
            }
        }
    }
    let mut sweep: Vec<Segment> = vec![];
    for i in 0..seg.len() {
        let (start, end) = seg[i];
        //dbg!(seg[i]);
        sweep.push(Segment::new(i, start, true));
        sweep.push(Segment::new(i, end, false));
    }
    sweep.sort_by(|&s1, &s2| f64cmp(s1.t, s2.t).then_with(|| if s1.left_endpoint { Ordering::Less } else { Ordering::Greater }));
    //dbg!(sweep.clone());
    use std::cmp::max;
    let mut sub_res = res;
    for s in sweep {
        if s.left_endpoint {
            sub_res += 1;
        } else {
            sub_res -= 1;
        }
        res = max(res, sub_res);
    }
    writeln!(sout, "{}", res).ok();
}
