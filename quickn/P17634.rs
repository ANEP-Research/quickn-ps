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

// (g, x, y)
fn gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, xp, yp) = gcd(b, a%b);
        (g, yp, xp - yp*(a/b))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let (mut a, mut b): (i128, i128) = (scan.token(), scan.token());
    let (g, x, y) = gcd(a, b);
    //dbg!(g);
    let b_inv = (a + (y%a))%a;
    let m = a*b;
    //dbg!((b_inv*b)%a);
    let mut sweep: Vec<(i128, i128)> = vec![];
    for i in 0..n {
        let (l_i, r_i): (i128, i128) = (scan.token(), scan.token());
        let len = r_i - l_i + 1;
        if len >= m {
            sweep.push((0, m-1));
        } else {
            let r1 = l_i % m;
            let r2 = r_i % m;
            if r1 <= r2 {
                sweep.push((r1, r2));
            } else {
                sweep.push((r1, m-1));
                sweep.push((0, r2));
            }
        }
    }
    sweep.sort_by(
        |&(l1, r1), &(l2, r2)| {
            l1.cmp(&l2).then_with(|| r1.cmp(&r2))
        }
    );
    let mut s = -1;
    let mut e = -2;
    let mut res = 0;
    for (l_i, r_i) in sweep {
        if l_i-e > 1 {
            res += e - s + 1;
            s = l_i;
        }
        e = r_i;
    }    
    res += e - s + 1;
    writeln!(sout, "{}", res).ok();
}
