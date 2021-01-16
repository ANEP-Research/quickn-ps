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

use std::collections::HashMap;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: i64 = scan.token();
    let mut hash: Vec<HashMap<Vec<i32>, i64>> = vec![HashMap::new();5];
    for _i in 0..n {
        let mut arr: [i32;5] = [0;5];
        for j in 0..5 {
            arr[j] = scan.token();
        }
        arr.sort();
        for j in 1..32 {
            let mut res: Vec<i32> = vec![];
            for bit in 0..5 {
                if (j & (1 << bit)) != 0 {
                    res.push(arr[bit]);
                }
            }
            if let Some(&t) = hash[res.len()-1].get(&res) {
                hash[res.len()-1].remove(&res);
                hash[res.len()-1].insert(res, t+1);
            } else {
                hash[res.len()-1].insert(res, 1);
            }
        }
    }
    let mut res = n*(n-1)/2;
    for i in 0..5 {
        let mut sub_res = 0;
        for (a, b) in hash[i].clone() {
            sub_res += b*(b-1)/2;
        }
        if i % 2 == 0 {
            res -= sub_res;
        } else {
            res += sub_res;
        }
    }
    writeln!(sout, "{}", res).ok();
}
