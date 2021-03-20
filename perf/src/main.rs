#![allow(dead_code)]
use std::{time::{Duration, Instant}, vec};

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

fn bench_strides() {
    println!("Allocating 128MiB array ...");
    let mut arr: Vec<i32> = vec![1; (1024 * 1024 * 128) / 4];
    println!("Allocated 128MiB array. Benching ...");

    // For each stride
    for j in &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024] {
        // Repeat 20 times
        for _ in 0..10 {
            let start = Instant::now();

            // Striding through the array
            for i in (0..arr.len()).step_by(*j) {
                arr[i] = arr[i].wrapping_mul(3);
            }
            println!("{}\t{}", j, (Instant::now() - start).as_millis())
        }
    }
}

fn time_it<F, T>(label: &str, f: F) -> T
where
    F: Fn() -> T,
    T: Default
{
    let mut result: T = T::default();
    let mut stats = Vec::with_capacity(1024);

    for _ in 0..50 {
        let start = Instant::now();
        result = f();
        stats.push(Instant::now() - start);
    }

    stats.sort();
    let p50 = if stats.len() % 2 == 0 {
        let upper = stats[stats.len() / 2];
        let lower = stats[(stats.len() / 2) - 1];
        (lower + upper) / 2
    } else {
        stats[stats.len() / 2]
    };
    let avg: Duration = stats.iter().sum::<Duration>() / stats.len() as u32;

    println!("[{}] p50 duration: {}us", label, p50.as_micros());
    println!("[{}] avg duration: {}us", label, avg.as_micros());
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

fn matrix_demo() {
    let m = Matrix::new(L3_LINES, LINE_SIZE);
    show_sizes(m.memsize());
    println!("sum: {}", time_it("row major", || sum_matrix(&m, true)));
    println!("sum: {}", time_it("col major", || sum_matrix(&m, false)));
}

fn main() {
    bench_strides();
}
