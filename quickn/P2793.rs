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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (a, b): (i64, i64) = (scan.token(), scan.token());
    let mut prefix_lcm: Vec<i64> = vec![0, 1];
    let mut len: Vec<i64> = vec![0, 0];
    let mut res: i64 = 0;
    for p in 2..44 {
        prefix_lcm.push((prefix_lcm[(p-1) as usize]*p)/gcd(prefix_lcm[(p-1) as usize], p));
        let mut cur_len = 1;
        for q in 2..p {
            if p % q != 0 {
                cur_len = len[q as usize] + 1;
                break;
            }
        }
        len.push(cur_len);
        res += (cur_len+1)*((b/prefix_lcm[(p-1) as usize]) - ((a-1)/prefix_lcm[(p-1) as usize]));
        res -= (cur_len+1)*((b/prefix_lcm[p as usize]) - ((a-1)/prefix_lcm[p as usize]));
    }
    writeln!(sout, "{}", res).ok();
}
