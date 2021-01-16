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

/*
    [ SOLUTION BOJ 2441 by quickn ]
    TIME COMPLEXITY: O(\sqrt{L/G}log\sqrt{L/G})
    SPACE COMPLEXITY: O(\log{\sqrt{L/G}})
    You can use this solution code by CC0 License
*/

fn fast_pow_mod(a: i64, b: usize) -> i64 {
    let (mut r, mut a_t, mut b_t) = (1, a, b);
    while b_t > 0 {
        if b_t % 2 == 1 {
            r *= a_t;
            r %= MOD;
        }
        a_t *= a_t;
        a_t %= MOD;
        b_t /= 2;
    }
    r
}

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, g, l): (usize, i64, i64) = (scan.token(), scan.token(), scan.token());
        // CONDIDITON: Existence of a solution
        // --- L mod G = 0
        if l % g == 0 {
            // We can do prime factorization of the L/G
            // Let L/G = \Prod_{i} p_{i}^{e_{i}}
            // We assume that all elements of the sequnce have G
            // For each p_{i}, at least once of the element must haven't factor p_{i}
            // However, at least once of the element must have p_{i}^{e_{i}}
            // We define A1 to 'a set of sequnces where all elements dosen't have p_{i}^{e_{i}}' and A2 to 'a set of sequnce where all elements have factor p_{i}'
            // Since number of the case of this is (e_{i}+1)^{N} - |A1| - |A2| + |A1 \cup A2|
            // But |A1| = e_{i}^{N} = |A2| and |A1 \cup A2| = (e_{i}-1)^{N}
            // Thus we have (e_{i}+1)^{N} - 2e_{i}^{N} + (e_{i}-1)^{N}
            let mut ans = 1;
            let mut new_l = l / g;
            let mut target = new_l;
            let mut p = 2;
            let mut factors: Vec<(i64, i64)> = vec![];
            while p * p <= new_l {
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
                ans *= (fast_pow_mod(e + 1, n) - 2 * fast_pow_mod(e, n) + fast_pow_mod(e - 1, n))
                    % MOD;
                ans %= MOD;
                ans += MOD;
                ans %= MOD;
            }
            writeln!(sout, "{}", ans).ok();
        } else {
            // FAILED: Not exists a solution
            writeln!(sout, "{}", 0).ok();
        }
    }
}
