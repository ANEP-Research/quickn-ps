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

use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Island,
    Shallow,
    Forbidden,
}

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];

#[derive(Clone, Debug)]
struct DFS {
    r: usize,
    c: usize,
    arr: Vec<Vec<Block>>,
    visited: Vec<Vec<bool>>,
    res: Vec<(usize, usize)>,
}

impl DFS {
    fn new(r: usize, c: usize, arr: Vec<Vec<Block>>) -> Self {
        Self {
            r,
            c,
            arr,
            visited: vec![vec![false; c]; r],
            res: vec![],
        }
    }

    fn dfs1(&mut self, i: usize, j: usize) {
        if !self.visited[i][j] && self.arr[i][j] == Block::Island {
            self.visited[i][j] = true;
            self.res.push((i, j));
            for k in 0..4 {
                let (new_i, new_j) = ((i as i32) + DX[k], (j as i32) + DY[k]);
                if new_i >= 0 && new_j >= 0 && new_i < (self.r as i32) && new_j < (self.c as i32) {
                    if self.arr[new_i as usize][new_j as usize] == Block::Island {
                        self.dfs1(new_i as usize, new_j as usize);
                    }
                }
            }
        }
    }
}

const MAX: usize = 17;

#[derive(Clone, Debug)]
struct DP {
    n: usize,
    dp: Vec<Vec<i32>>,
    d: Vec<Vec<i32>>,
    visited: Vec<Vec<bool>>,
}

impl DP {
    fn new(n: usize, d: Vec<Vec<i32>>) -> Self {
        Self {
            n,
            dp: vec![vec![std::i32::MAX; 1 << n]; n],
            d,
            visited: vec![vec![false; 1 << n]; n],
        }
    }

    fn dp(&mut self, i: usize, mask: usize) -> i32 {
        if self.visited[i][mask] {
            self.dp[i][mask]
        } else {
            self.visited[i][mask] = true;
            let mut res = std::i32::MAX;
            let new_mask = mask ^ (1 << i);
            if new_mask == 0 {
                res = 0;
            } else {
                for j in 0..MAX {
                    if ((1 << j) & new_mask) != 0 {
                        if self.d[j][i] != std::i32::MAX {
                            let r = self.dp(j, new_mask);
                            if r != std::i32::MAX {
                                res = min(res, r + self.d[j][i]);
                            }
                        }
                    }
                }
            }
            self.dp[i][mask] = res;
            res
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (r, c): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<Vec<Block>> = vec![vec![Block::Forbidden; c]; r];
    for i in 0..r {
        let s: String = scan.token();
        let s_arr: Vec<char> = s.chars().collect();
        for j in 0..c {
            if s_arr[j] == 'S' {
                arr[i][j] = Block::Shallow;
            } else if s_arr[j] == 'X' {
                arr[i][j] = Block::Island;
            }
        }
    }
    let mut dfs = DFS::new(r, c, arr.clone());
    let mut components: Vec<Vec<(usize, usize)>> = vec![];
    let mut f: Vec<usize> = vec![r * c; r * c];
    for i in 0..r {
        for j in 0..c {
            if !dfs.visited[i][j] && dfs.arr[i][j] == Block::Island {
                dfs.dfs1(i, j);
                components.push(dfs.res.clone());
                dfs.res.clear();
            }
        }
    }
    let co = components.len();
    for i in 0..co {
        for (x, y) in components[i].clone() {
            f[(x * c) + y] = i;
        }
    }
    use std::collections::VecDeque;
    let mut dist = vec![vec![std::i32::MAX; co]; co];
    for i in 0..co {
        dist[i][i] = 0;
    }
    for i in 0..co {
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false;c];r];
        q.push_back((components[i][0].0, components[i][0].1, 0));
        visited[components[i][0].0][components[i][0].1] = true;
        while let Some((x, y, d)) = q.pop_front() {
            for k in 0..4 {
                let (new_i, new_j) = ((x as i32) + DX[k], (y as i32) + DY[k]);
                if new_i >= 0 && new_j >= 0 && new_i < (r as i32) && new_j < (c as i32) {
                    if !visited[new_i as usize][new_j as usize] {
                        if arr[new_i as usize][new_j as usize] == Block::Island {
                            let func = ((new_i as usize)*c) + (new_j as usize);
                            dist[i][f[func]] = min(dist[i][f[func]],d);
                            visited[new_i as usize][new_j as usize] = true;
                            q.push_back((new_i as usize, new_j as usize, d));
                        } else if arr[new_i as usize][new_j as usize] == Block::Shallow {
                            visited[new_i as usize][new_j as usize] = true;
                            q.push_back((new_i as usize, new_j as usize, d+1));
                        }
                    }
                }
            }
        }
    }
    let mut dp = DP::new(co, dist.clone());
    let mut res = std::i32::MAX;
    for i in 0..co {
        res = min(res, dp.dp(i, (1 << co) - 1));
    }
    writeln!(sout, "{}", res).ok();
}
