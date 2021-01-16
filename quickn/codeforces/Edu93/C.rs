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

use std::collections::{HashMap, HashSet};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let s: String = scan.token();
        let arr_s: Vec<char> = s.chars().collect();
        let mut arr: Vec<i64> = vec![];
        for a in arr_s {
            arr.push(((a as u8) - ('0' as u8)) as i64);
        }
        let mut hash: HashMap<i64, i64> = HashMap::new();
        let mut set: HashSet<i64> = HashSet::new();
        let mut sum = 0;
        hash.insert(0, 1);
        for i in 0..n {
            sum += arr[i];
            let a = (sum - ((i+1) as i64));
            if let Some(&t) = hash.get(&a) {
                hash.remove(&a);
                hash.insert(a, t+1);
            } else {
                hash.insert(a, 1);
            }
            set.insert(a);
        }
        let mut ans = 0;
        for a in set {
            let x = hash.get(&a).unwrap();
            ans += (x*x - x)/2;
        }
        writeln!(sout, "{}", ans).ok();
    }
}
