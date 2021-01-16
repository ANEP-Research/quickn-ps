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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Triangle {
    p: Point,
    h: i64,
}

impl Triangle {
    fn new(p: Point, h: i64) -> Self {
        Self {
            p,
            h,
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            p: Point::new(0, 0),
            h: 2_000_000_000,
        }
    }
}

struct Intersection {
    n: usize,
    arr: Vec<Triangle>,
    area: Vec<f64>,
    res: f64,
}

impl Intersection {
    fn new(n: usize, arr: Vec<Triangle>) -> Self {
        Self {
            n,
            arr,
            area: vec![0.0;(1 << n)],
            res: 0.0,
        }
    }

    fn func(&mut self, odd: bool, mask: usize, i: usize, tri: Triangle) {
        let x_d = (self.arr[i].p.x + self.arr[i].h - tri.p.x);
        let y_d = (self.arr[i].p.y + self.arr[i].h - tri.p.y);
        if x_d >= 0 && x_d <= tri.h && y_d >= 0 && y_d <= tri.h {
            let new_p = Point::new(max(self.arr[i].p.x, tri.p.x), max(self.arr[i].p.y, tri.p.y));
            let new_h = ((self.arr[i].p.x + self.arr[i].h - new_p.x).abs() - (self.arr[i].p.y - new_p.y).abs()).abs();
            self.area[mask] = ((new_h * new_h) as f64)/2.0;
            //dbg!(new_p, mask, new_h);
            if odd {
                self.res += self.area[mask];
            } else {
                self.res -= self.area[mask];
            }
            for j in 0..self.n {
                if (mask & (1 << j)) == 0 {
                    self.func(!odd, mask | (1 << j), j, Triangle::new(new_p, new_h));
                }
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<Triangle> = vec![Triangle::new(Point::new(0, 0), 0);n];
    let mut sub_res: f64 = 0.0;
    for i in 0..n {
        let (x, y, r): (i32, i32, i32) = (scan.token(), scan.token(), scan.token());
        arr[i] = Triangle::new(Point::new(x as i64, y as i64), r as i64);
        sub_res += ((r*r) as f64)/2.0;
    }
    let mut int = Intersection::new(n, arr);
    for i in 0..n {
        int.func(true, (1 << i), i, Triangle::default());
    }
    writeln!(sout, "{:.1}", int.res).ok();
}
