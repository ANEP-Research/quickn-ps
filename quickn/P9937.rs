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

use std::collections::{BTreeSet, BTreeMap};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Frac {
    sign: bool,
    a: i32,
    b: i32,
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let k = (self.a as i64)*(other.b as i64) - (self.b as i64)*(other.a as i64);
        Some(if k < 0 {
            Ordering::Less
        } else if k == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        })
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Self) -> Ordering {
        let k = self.s()*(self.a as i64)*(other.b as i64) - other.s()*(self.b as i64)*(other.a as i64);
        if k < 0 {
            Ordering::Less
        } else if k == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

use std::mem::swap;
use std::cmp::max;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        max(a,b)
    } else {
    let (mut a_t, mut b_t) = (a, b);
    while b_t != 0 {
        a_t %= b_t;
        swap(&mut a_t, &mut b_t);
    }
    a_t
    }
}

impl Frac {
    fn new(a: i32, b: i32) -> Self {
        let s = (a as i64)*(b as i64);
        let sign = if s >= 0 {
            false
        } else {
            true
        };
        let mut g = gcd(a.abs(), b.abs());
        if g == 0 { g = 1; }
        if b == 0 {
            Self {
                sign: false,
                a: 1,
                b: 0,
            }
        } else if a == 0 {
            Self {
                sign: false,
                a: 0,
                b: 1,
            }
        } else {
        Self {
            sign,
            a: a.abs()/g,
            b: b.abs()/g,
        }
        }   
    }

    fn s(&self) -> i64 {
        if self.sign {
            -1
        } else {
            1
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Seg {
    a: i32,
    b: i32,
    c: i32,
}

impl Seg {
    fn new(a: i32, b: i32, c: i32) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    fn tangent(&self) -> Frac {
        Frac::new(-self.a, self.b)
    }
}

use std::cmp::Ordering;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut _seg: BTreeSet<Seg> = BTreeSet::new();
    for i in 0..n {
        let (a, b, c): (i32, i32, i32) = (scan.token(), scan.token(), scan.token());
        let mut g = gcd(a.abs(),gcd(b.abs(),c.abs()));
        if g == 0 { g = 1; };
        _seg.insert(Seg::new(a/g, b/g, c/g));
    }
    let mut seg: Vec<Seg> = _seg.into_iter().collect();
    seg.sort_by(|&s1, &s2| {
        let u = s1.tangent();
        let t = s2.tangent();
        u.cmp(&t)
    });
    //dbg!(gcd(188,2));
    //seg.reverse();
    let mut t: i64 = 0;
    let mut hash: BTreeMap<Frac, i64> = BTreeMap::new();
    let mut res: i64 = 0;
    for i in 0..seg.len() {
        let tan = seg[i].tangent();
        if let Some(&v) = hash.get(&tan) {
            hash.remove(&tan);
            hash.insert(tan, v+1);
        } else {
            hash.insert(tan, 1);
        }
    }
    for i in 0..seg.len() {
        let tan = seg[i].tangent();
        if let Some(&v) = hash.get(&tan) {
            t += v;
        }
    }
    let n2 = seg.len();
    dbg!(n2);
    for i in 0..n2 {
        let tan = seg[i].tangent();
        if let Some(&u) = hash.get(&tan) {
            let exclude = (n2 as i64) - u;
            res += exclude*exclude - t + (u*u);
        }
    }
    writeln!(sout, "{}", res/6).ok();
}
