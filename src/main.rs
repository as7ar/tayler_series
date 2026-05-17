use std::f64::consts::PI;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Row {
    degree: i64,
    sin_real: f64,
    sin_approx: f64,
    sin_error: f64,
    cos_real: f64,
    cos_approx: f64,
    cos_error: f64,
}

pub struct Func<F>
where
    F: Fn(f64, i64) -> f64,
{
    func: F,
}

impl<F> Func<F>
where
    F: Fn(f64, i64) -> f64,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }

    pub fn call(&self, x: f64, n: i64) -> f64 {
        (self.func)(x, n)
    }
}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}

fn sum<F>(k: i64, n: i64, x: f64, f: &Func<F>) -> f64
where
    F: Fn(f64, i64) -> f64,
{
    let mut i = k;
    let mut result = 0.0;

    while i <= n {
        result += f.call(x, i);
        i += 1;
    }

    result
}

fn main() {
    let sin_func = Func::new(|x, n| {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let power = 2 * n + 1;

        sign * x.powi(power as i32) / factorial(power) as f64
    });

    let cos_func = Func::new(|x, n| {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let power = 2 * n;

        sign * x.powi(power as i32) / factorial(power) as f64
    });

    let x = (11.0 * PI) / 6.0;

    let mut rows = Vec::new();

    for i in 1..10 {
        let sin_approx = sum(0, i, x, &sin_func);
        let cos_approx = sum(0, i, x, &cos_func);

        let sin_real = x.sin();
        let cos_real = x.cos();

        rows.push(Row {
            degree: i,
            sin_real,
            sin_approx,
            sin_error: (sin_real - sin_approx).abs(),
            cos_real,
            cos_approx,
            cos_error: (cos_real - cos_approx).abs(),
        });
    }

    let table = Table::new(rows);

    println!("{}", table);
}