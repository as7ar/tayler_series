use std::f64::consts::PI;

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
        result += (f.func)(x, i);
        i += 1;
    }

    result
}

fn main() {

    let sin = Func::new(|x, n| {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let power = 2 * n + 1;
        sign * x.powi(power as i32) / factorial(power) as f64
    });

    let cos = Func::new(|x, n| {
        let sign = if n%2==0 {1.0} else {-1.0};
        let power = 2*n;
        sign*x.powi(power as i32) / factorial(power) as f64
    });
    
    let x = (13f64*PI)/6f64;
    
    println!("각: {}", x);
    println!("sin: {}, cos: {}", x.sin(), x.cos());
    
    for i in 1..10 {
        let result_sin = sum(0, i, x, &sin);
        let result_cos = sum(0, i, x, &cos);

        println!("k={}까지", i);
        println!("sin근사: {}, 오차: {}", result_sin, (x.sin()-result_sin).abs());
        println!("cos근사: {}, 오차: {}", result_cos, (x.cos()-result_cos).abs());
    }
}