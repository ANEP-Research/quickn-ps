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

const MAX: usize = 1_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let mut arr: Vec<usize> = vec![0;n];
        let mut cnt: Vec<usize> = vec![0;MAX+1];
        let mut adj: Vec<usize> = vec![0;MAX+1];
        for i in 0..n {
            arr[i] = scan.token();
            cnt[arr[i]] += 1;
        }
        for a in 1..=MAX {
            if cnt[a] >= 1 {
                let mut a_t = a as i64;
                let mut p = 2;
                let mut primes: Vec<(i64, usize)> = Vec::new();
                while p <= a_t {
                    let mut facts = 0;
                    while a_t % p == 0 {
                        a_t /= p;
                        facts += 1;
                    }
                    if facts != 0 {
                        primes.push((p, facts));
                    }
                    p += 1;
                }
                let mut t = 1;
                for (q, tries) in primes {
                    if tries % 2 == 1 {
                        t *= q;
                    }
                }
                let mut i = t;
                while i <= (MAX as i64) {
                    let prod = (i*(a as i64));
                    if ((prod as f64).sqrt() as i64) == prod/((prod as f64).sqrt() as i64) {
                        adj[a] += cnt[i as usize];
                    }
                    i += t;
                }
            }
        }
        let mut f1 = 0;
        let mut evens = 0;
        let mut f2_cand = 0;
        for i in 1..=MAX {
            if adj[i] % 2 == 0 {
                if adj[i] >= 1 {
                    evens += 1;
                }
            } else {
                f2_cand = max(f2_cand, adj[i]);
            }
            f1 = max(f1, adj[i]);
        }
        let q: usize = scan.token();
        for i in 0..q {
            let w: i64 = scan.token();
            if w == 0 {
                writeln!(sout, "{}", f1).ok();
            } else {
                writeln!(sout, "{}", max(f2_cand, evens)).ok();
            }
        }
    }
}
