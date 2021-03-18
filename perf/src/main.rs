use std::{
    // mem::{size_of, size_of_val},
    time::Instant,
};

struct Matrix {
    rows: usize,
    cols: usize,
    m: Vec<i8>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        return Self {
            rows,
            cols,
            m: vec![1; rows * cols],
        };
    }

    pub fn get(&self, row: usize, col: usize) -> i8 {
        self.m[row * self.cols + col]
    }

    pub fn memsize(&self) -> usize {
        self.m.len()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

fn sum_matrix(m: &Matrix, row_major: bool) -> i64 {
    let mut sum: i64 = 0;

    if row_major {
        for r in 0..m.rows() {
            for c in 0..m.cols() {
                sum += m.get(r, c) as i64;
            }
        }
    } else {
        for c in 0..m.cols() {
            for r in 0..m.rows() {
                sum += m.get(r, c) as i64;
            }
        }
    }

    sum
}

fn time_it<F, T>(label: &str, f: F) -> T
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result: T = f();
    let d = Instant::now() - start;
    println!("[{}] duration: {}us", label, d.as_micros());
    result
}

const LINE_SIZE: usize = 64;
const L1_SIZE: usize = 32768;
const L1_LINES: usize = L1_SIZE / LINE_SIZE;
const L2_SIZE: usize = 262144;
const L2_LINES: usize = L2_SIZE / LINE_SIZE;
const L3_SIZE: usize = 8388608;
const L3_LINES: usize = L3_SIZE / LINE_SIZE;

fn show_sizes(bytes: usize) {
    println!("memsize = {} bytes", bytes);
    println!("Num L1 cache lines = {} r{} of {}", bytes / LINE_SIZE, bytes % LINE_SIZE, L1_LINES);
    println!("% of L1 = {}%", (bytes as f64 / L1_SIZE as f64) * 100.0);
    println!("% of L2 = {}%", (bytes as f64 / L2_SIZE as f64) * 100.0);
    println!("% of L3 = {}%", (bytes as f64 / L3_SIZE as f64) * 100.0);
}

fn main() {
    let m = Matrix::new(L3_LINES, LINE_SIZE);
    show_sizes(m.memsize());
    println!("sum: {}", time_it("row major", || sum_matrix(&m, true)));
    println!("sum: {}", time_it("col major", || sum_matrix(&m, false)));
}
