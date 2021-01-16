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

fn char2a(a: char) -> u8 {
    (a as u8) - ('a' as u8)
}

fn u8toa(a: u8) -> char {
    (a + ('a' as u8)) as char
}

use std::collections::HashSet;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let mut s = s2vec(scan.token());
        let n = s.len();
        let mut hash: HashSet<usize> = HashSet::new();
        for i in 1..n {
            if i > 1 && s[i] == s[i-2] {
                if hash.contains(&(i-2)) {
                    while s[i] == s[i-2] {
                        s[i-2] = u8toa((char2a(s[i-2]) + 1)%26);
                    }
                } else {
                    hash.insert(i);
                    while s[i] == s[i-2] {
                        s[i] = u8toa((char2a(s[i]) + 1)%26);
                    }
                }
            }
            if s[i] == s[i-1] {
                if hash.contains(&(i-1)) {
                    while s[i] == s[i-1] {
                        s[i-1] = u8toa((char2a(s[i-1]) + 1)%26);
                    }
                } else {
                    hash.insert(i);
                    while s[i] == s[i-1] {
                        s[i] = u8toa((char2a(s[i]) + 1)%26);
                    }
                }
            }
        }
        writeln!(sout, "{}", hash.len()).ok();
    }
}
