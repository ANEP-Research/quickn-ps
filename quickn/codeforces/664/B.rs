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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m, s_x, s_y): (usize, usize, usize, usize) = (scan.token(), scan.token(), scan.token(), scan.token());
    let mut res: Vec<(usize, usize)> = vec![];
    let (mut x, mut y) = (s_x, s_y);
    while y > 0 {
        res.push((x, y));
        y -= 1;
    }
    y = s_y + 1;
    while y <= m {
        res.push((x, y));
        y += 1;
    }
    y = m;
    x += 1;
    let mut dir = false;
    while x <= n {
        if !dir {
            while y > 0 {
                res.push((x, y));
                y -= 1;
            }
            y = 1;
        } else {
            while y <= m {
                res.push((x, y));
                y += 1;
            }
            y = m;
        }
        dir = !dir;
        x += 1;
    }
    x = s_x-1;
    while x > 0 {
        if !dir {
            while y > 0 {
                res.push((x, y));
                y -= 1;
            }
            y = 1;
        } else {
            while y <= m {
                res.push((x, y));
                y += 1;
            }
            y = m;
        }
        dir = !dir;
        x -= 1;
    }
    for (x, y) in res {
        writeln!(sout, "{} {}", x, y).ok();
    }
}
