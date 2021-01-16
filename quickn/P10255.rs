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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CCWResult {
    Left,
    Right,
    Same,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
        }
    }

    fn ccw(&self, other: Self) -> CCWResult {
        let res = self.x*other.y - self.y*other.x;
        if res > 0 {
            CCWResult::Left
        } else if res == 0 {
            CCWResult::Same
        } else {
            CCWResult::Right
        }
    }
}

use std::ops::{Add, Sub};
use std::collections::BTreeSet;

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    ll: Vec2,
    rr: Vec2,
}

impl Rectangle {
    fn new(x_min: i64, y_min: i64, x_max: i64, y_max: i64) -> Self {
        Self {
            ll: Vec2::new(x_min, y_min),
            rr: Vec2::new(x_max, y_max),
        }
    }

    fn h(&self) -> i64 {
        self.rr.y - self.ll.y
    }

    fn lr(&self) -> Vec2 {
        self.ll + Vec2::new(0, self.h())
    }

    fn rl(&self) -> Vec2 {
        self.rr - Vec2::new(0, self.h())
    }
}

use std::cmp::{min, max};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (x1, y1, x2, y2, mut x3, mut y3, mut x4, mut y4) = (scan.token(), scan.token(), scan.token(), scan.token(), scan.token(), scan.token(), scan.token(), scan.token());
        let r = Rectangle::new(x1, y1, x2, y2);
        let (mut s1, mut s2) = (Vec2::new(x3, y3), Vec2::new(x4, y4));
        if x3 > x4 {
            let s = s1.clone();
            s1 = s2;
            s2 = s;
        } else if x3 == x4 {
            if y3 > y4 {
                let s = s1.clone();
                s1 = s2;
                s2 = s;
            }
        }
        let mut points = [r.lr(), r.ll, r.rl(), r.rr];
        let mut res: usize = 0;
        let mut sub_res: usize = 0;
        let mut q: BTreeSet<Vec2> = BTreeSet::new();
        for i in 0..4 {
            let j = (i + 1)%4;
            let t1 = s1 - points[i];
            let t2 = s2 - points[i];
            let t3 = s1 - points[j];
            let t4 = s2 - points[j];
            let t5 = s1 - points[i];
            let t6 = s1 - points[j];
            let t7 = s2 - points[i];
            let t8 = s2 - points[j];
            let c1 = t1.ccw(t2);
            let c2 = t3.ccw(t4);
            let c3 = t5.ccw(t6);
            let c4 = t7.ccw(t8);
            if (c1 != c2) && (c3 != c4) {
                if c1 == CCWResult::Same {
                    q.insert(points[i]);
                } else if c2 == CCWResult::Same {
                    q.insert(points[j]);
                } else {
                    res += 1;
                }
            } else if c1 == CCWResult::Same && c2 == CCWResult::Same && c3 == CCWResult::Same && c4 == CCWResult::Same {
                res = 4;
                break;
            }
        }
        if res != 4 {
            res += q.len();
        }
        writeln!(sout, "{}", res).ok();
    }
}
