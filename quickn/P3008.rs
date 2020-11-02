/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
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
struct Vec2 {
    x: i32,
    y: i32,
}

enum CCWResult {
    Right,
    Left,
    Same,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Frac {
    sign: bool,
    a: u32,
    b: u32,
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a_t, mut b_t) = (a, b);
    while b_t != 0 {
        a_t %= b_t;
        a_t ^= b_t;
        b_t ^= a_t;
        a_t ^= b_t;
    }
    a_t
}

impl Frac {
    fn new(a: i32, b: i32) -> Self {
        let g = gcd(a.abs(), b.abs());
        Self {
            sign: if (a as i64)*(b as i64) >= 0 { false } else { true },
            a: (a.abs()/g) as u32,
            b: (b.abs()/g) as u32,
        }
    }

    fn inverse(&self) -> Self {
        Self {
            sign: !self.sign,
            a: self.b,
            b: self.a,
        }
    }
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

use std::ops::Sub;

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

use std::collections::HashMap;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<Vec2> = vec![Vec2::new(0, 0); n];
    for i in 0..n {
        arr[i] = Vec2::new(scan.token(), scan.token());
    }
    let mut cnt = 0;
    for i in 0..n {
        let mut arr_i = arr.clone();
        let p0 = arr[i];
        arr_i.remove(i);
        let mut hash: HashMap<Frac, i32> = HashMap::new();
        let mut f_arr: Vec<Frac> = vec![];
        let (mut g1, mut g2) = (0, 0);
        for i in 0..(n-1) {
            let t = arr_i[i] - p0;
            let f = Frac::new(t.y, t.x);
            f_arr.push(f);
            if let Some(m) = hash.get_mut(&f) {
                *m += 1;
            } else {
                hash.insert(f, 1);
            }
        }
        for i in 0..(n-1) {
            if f_arr[i].b == 0 {
                g1 += 1;
            } else if f_arr[i].a == 0 {
                g2 += 1;
            } else {
                if let Some(c1) = hash.get(&f_arr[i]) {
                    if c1.clone() > 0 {
                        if let Some(c2) = hash.get(&f_arr[i].inverse()) {
                            cnt += c2.clone();
                        }
                        if let Some(c3) = hash.get_mut(&f_arr[i]) {
                            *c3 -= 1;
                        }
                    }
                }
            }
        }
        cnt += g1*g2;
    }
    writeln!(sout, "{}", cnt).ok();
}