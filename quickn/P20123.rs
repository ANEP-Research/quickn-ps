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

fn or(dest: &mut Vec<Vec<char>>, source: &Vec<Vec<char>>, off: (usize, usize)) {
    for i in 0..min(source.len(), dest.len() - off.0) {
        for j in 0..min(source.len(), dest.len() - off.1) {
            let x = i + off.0;
            let y = j + off.1;
            if source[i][j] != ' ' {
                dest[x][y] = source[i][j];
            }
        }
    }
}

// d
// 1 - b
// 2 - p
// 3 - q
// 4 - d
fn get(u: usize, d: usize) -> Vec<Vec<char>> {
    if u == 1 {
        if d == 1 {
            vec![
                vec![
                    '|', ' '
                ],
                vec![
                    'b', '-'
                ]
            ]
        } else if d == 2 {
            vec![
                vec![
                    'p', '-'
                ],
                vec![
                    '|', ' '
                ]
            ]
        } else if d == 3 {
            vec![
                vec![
                    '-', 'q'
                ],
                vec![
                    ' ', '|'
                ]
            ]
        } else {
            vec![
                vec![
                    ' ', '|'
                ],
                vec![
                    '-', 'd'
                ]
            ]
        }
    } else {
        let n = 1<<u;
        let mut res: Vec<Vec<char>> = vec![vec![' ';n];n];
        let d1 = get(u-1, 1);
        let d2 = get(u-1, 2);
        let d3 = get(u-1, 3);
        let d4 = get(u-1, 4);
        if d == 1 {
            or(&mut res, &d2, (0, 0));
            or(&mut res, &d1, (n/2, 0));
            or(&mut res, &d1, (n/4, n/4));
            or(&mut res, &d4, (n/2, n/2));
        } else if d == 2 {
            or(&mut res, &d2, (0, 0));
            or(&mut res, &d2, (n/4, n/4));
            or(&mut res, &d1, (n/2, 0));
            or(&mut res, &d3, (0, n/2));
        } else if d == 3 {
            or(&mut res, &d2, (0, 0));
            or(&mut res, &d3, (n/4, n/4));
            or(&mut res, &d3, (0, n/2));
            or(&mut res, &d4, (n/2, n/2));
        } else {
            or(&mut res, &d3, (0, n/2));
            or(&mut res, &d4, (n/4, n/4));
            or(&mut res, &d1, (n/2, 0));
            or(&mut res, &d4, (n/2, n/2));
        }
        res
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut res: Vec<Vec<char>> = vec![vec![' ';n];n];
    let mut off = 0;
    let mut size = 2;
    let mut u = 1;
    while size <= n {
        let (mut x, mut y) = (off, 0);
        let block = get(u, 1);
        while x <= n-size {
            or(&mut res, &block, (x, y));
            x += size;
            y += size;
        }
        off += size;
        u += 1;
        size <<= 1;
    }
    for i in 0..n {
        for j in 0..n {
            write!(sout, "{} ", res[i][j]).ok();
        }
        write!(sout, "\n").ok();
    }
}
