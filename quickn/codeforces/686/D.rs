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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: i64 = scan.token();
        let mut i = 1;
        let mut res: Vec<i64> = vec![];
        while i*i <= n {
            if n % i == 0 {
                let i2 = n/i;
                if i == i2 {
                    res.push(i);
                } else {
                    if i != 1 {
                        res.push(i);
                    }
                    if i2 != 1 {
                        res.push(i2);
                    }
                }
            }
            i += 1;
        }
        res.sort();
        let mut res2: Vec<i64> = vec![];
        let mut n_t = n;
        let mut cnt: Vec<(i64, i64)> = vec![];
        let mut clen = 0;
        for i in 0..res.len() {
            if n_t % res[i] == 0 {
                cnt.push((res[i], 0));
                clen += 1;
            }
            while n_t % res[i] == 0 {
                n_t /= res[i];
                cnt[clen-1].1 += 1;
            }
        }
        cnt.sort_by(|&(p1, c1), &(p2, c2)| c1.cmp(&c2).then_with(|| p1.cmp(&p2)));
        let k = cnt[clen-1].1;
        let mut prev = 1;
        for i in 0..k {
            for (pi, ci) in cnt.clone() {
                if k - ci == i {
                    prev *= pi;
                }
            }
            res2.push(prev);
        }
        writeln!(sout, "{}", k).ok();
        for a in res2 {
            write!(sout, "{} ", a).ok();
        }
        write!(sout, "\n",).ok();
    }
}
