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

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a_t, mut b_t) = (a, b);
    while b_t != 0 {
        a_t %= b_t;
        a_t ^= b_t;
        b_t ^= a_t;
        a_t ^= b_t;
    }
    a_t
}

fn lcm(a: i64, b: i64) -> i64 {
    a*b / gcd(a, b)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, g, l): (i64, i64, i64) = (scan.token(), scan.token(), scan.token());
        assert_eq!(n, 3);
        let mut res: i64 = 0;
        for a in 1..=l {
            for b in 1..=l {
                for c in 1..=l {
                    if gcd(a, gcd(b, c)) == g && lcm(lcm(a, b),c) == l {
                        res += 1;
                    }
                }
            }
        }
        let mut res2: i64 = 1;
        let mut target = l/g;
        let mut p = 2;
        let mut factors: Vec<(i64, i64)> = vec![];
        while p*p <= l/g {
            let mut e = 0;
            while target % p == 0 {
                e += 1;
                target /= p;
            }
            if e != 0 {
                factors.push((p, e));
            }
            p += 1;
        }
        if target > 1 {
            factors.push((target, 1));
        }
        for (p, e) in factors {
            res2 *= (e+1).pow(3) - 2*e.pow(3) + (e-1).pow(3);
        }
        assert_eq!(res, res2);
        writeln!(sout, "{}", res).ok();
    }
}
