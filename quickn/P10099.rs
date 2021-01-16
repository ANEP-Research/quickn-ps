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

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];

use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vertex {
    cost: i32,
    x: i32,
    y: i32,
}

impl Vertex {
    fn new(cost: i32, x: i32, y: i32) -> Self {
        Self { cost, x, y }
    }
}

use std::cmp::Ordering;

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.cost
                .cmp(&other.cost)
                .then_with(|| self.x.cmp(&other.x))
                .then_with(|| self.y.cmp(&other.y)),
        )
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (mut r, mut c): (usize, usize) = (scan.token(), scan.token());
    r += 2;
    c += 2;
    let mut arr: Vec<Vec<bool>> = vec![vec![true; c]; r];
    let mut start: (i32, i32) = (0, 0);
    let mut goal: (i32, i32) = (0, 0);
    for i in 0..(r - 2) {
        let arr_is: String = scan.token();
        let arr_i: Vec<char> = arr_is.chars().collect();
        for j in 0..(c - 2) {
            if arr_i[j] == '#' {
                arr[i + 1][j + 1] = false;
            } else if arr_i[j] == 'S' {
                start = (i as i32 + 1, j as i32 + 1);
            } else if arr_i[j] == 'C' {
                goal = (i as i32 + 1, j as i32 + 1);
            }
        }
    }
    for i in 0..r {
        arr[i][0] = false;
        arr[i][c - 1] = false;
    }
    for j in 0..c {
        arr[0][j] = false;
        arr[r - 1][j] = false;
    }
    let mut min_dist: Vec<Vec<i32>> = vec![vec![std::i32::MAX; c]; r];
    let mut warp: Vec<Vec<Vec<(i32, i32)>>> =
        vec![vec![vec![(std::i32::MAX, std::i32::MAX); c]; r]; 4];
    let mut warp_dist: Vec<Vec<i32>> = vec![vec![std::i32::MAX; c]; r];
    for i in 1..(r - 1) {
        let mut j = 1;
        while j < c {
            while j < (c - 1) && !arr[i][j] {
                j += 1;
            }
            if j >= (c - 1) {
                break;
            }
            let st = j;
            while j < (c - 1) && arr[i][j] {
                j += 1;
            }
            j -= 1;
            let ed = j;
            for k in (st + 1)..ed {
                warp[0][i][k] = (i as i32, st as i32);
                warp[1][i][k] = (i as i32, ed as i32);
            }
            if st != ed {
                warp[1][i][st] = (i as i32, ed as i32);
                warp[0][i][ed] = (i as i32, st as i32);
            }
            j += 1;
        }
    }
    for j in 1..(c - 1) {
        let mut i = 1;
        while i < (r - 1) {
            while i < (r - 1) && !arr[i][j] {
                i += 1;
            }
            if i >= (r - 1) {
                break;
            }
            let st = i;
            while i < (r - 1) && arr[i][j] {
                i += 1;
            }
            i -= 1;
            let ed = i;
            for k in (st + 1)..ed {
                warp[2][k][j] = (st as i32, j as i32);
                warp[3][k][j] = (ed as i32, j as i32);
            }
            if st != ed {
                warp[3][st][j] = (ed as i32, j as i32);
                warp[2][ed][j] = (st as i32, j as i32);
            }
            i += 1;
        }
    }
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    for i in 0..r {
        for j in 0..c {
            if !arr[i][j] {
                warp_dist[i][j] = 0;
                q.push_back((i as i32, j as i32));
            }
        }
    }
    while let Some((x, y)) = q.pop_front() {
        for k in 0..4 {
            let (new_x, new_y) = (x + DX[k], y + DY[k]);
            if new_x >= 0
                && new_x < r as i32
                && new_y >= 0
                && new_y < c as i32
                && warp_dist[new_x as usize][new_y as usize] == std::i32::MAX
            {
                warp_dist[new_x as usize][new_y as usize] = warp_dist[x as usize][y as usize] + 1;
                q.push_back((new_x, new_y));
            }
        }
    }
    let mut q: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    q.push((0, start.0, start.1));
    min_dist[start.0 as usize][start.1 as usize] = 0;
    while let Some((_cost, x, y)) = q.pop() {
        for i in 0..4 {
            let (new_x, new_y) = (x + DX[i], y + DY[i]);
            if new_x >= 0
                && new_x < r as i32
                && new_y >= 0
                && new_y < c as i32
                && arr[new_x as usize][new_y as usize]
            {
                let dist = min_dist[x as usize][y as usize] + 1;
                if min_dist[new_x as usize][new_y as usize] > dist {
                    min_dist[new_x as usize][new_y as usize] = dist;
                    q.push((-dist, new_x, new_y));
                }
            }
        }
        for i in 0..4 {
            let (new_x, new_y) = warp[i][x as usize][y as usize];
            if new_x >= 0 && new_x < r as i32 && new_y >= 0 && new_y < c as i32 {
                let dist = min_dist[x as usize][y as usize] + warp_dist[x as usize][y as usize];
                if min_dist[new_x as usize][new_y as usize] > dist {
                    min_dist[new_x as usize][new_y as usize] = dist;
                    q.push((-dist, new_x, new_y));
                }
            }
        }
    }
    writeln!(sout, "{}", min_dist[goal.0 as usize][goal.1 as usize]).ok();
}
