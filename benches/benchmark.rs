use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tarq::indicators::{bbands::BBands, sma::Sma, ema::Ema, vwma::Vwma};

use tarq::Indicator;
use tarq::enums::MovingAverage;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;

// Generates a vector of n random f64 values using a fixed seed.
fn generate_random_data(n: usize, seed: u64) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.random::<f64>()).collect()
}

fn benchmark_sma(c: &mut Criterion) {
    let data = generate_random_data(5_000_000, 42);
    let period = 50;
    let mut sma = Sma::new(black_box(&data), period).unwrap();
    c.bench_function("SMA", |b| {
        b.iter(|| {
            let _ = sma.calculate().unwrap();
        })
    });
}

fn benchmark_ema(c: &mut Criterion) {
    let data = generate_random_data(5_000_000, 42);
    let period = 50;
    let mut ema = Ema::new(black_box(&data), period).unwrap();

    c.bench_function("EMA", |b| {
        b.iter(|| {
            let _ = ema.calculate().unwrap();
        })
    });
}

fn benchmark_vwma(c: &mut Criterion) {
    let data = generate_random_data(5_000_000, 42);
    // Generate a separate random volume vector (using a different seed) for variety.
    let volume = generate_random_data(5_000_000, 84);
    let period = 50;
    let mut vwma = Vwma::new(black_box(&data), black_box(&volume), period).unwrap();
    c.bench_function("VWMA", |b| {
        b.iter(|| {
            let _ = vwma.calculate().unwrap();
        })
    });
}

fn benchmark_bbands(c: &mut Criterion) {
    let data = generate_random_data(5_000_000, 42);
    let period = 50;
    let std_dev = 2.0;

    let sma = Sma::new(&data, period).unwrap();
    let ma_type = MovingAverage::SMA(sma);


    let mut bb = BBands::new(black_box(&data), period, std_dev, ma_type).unwrap();

    c.bench_function("BBands", |b| {
        b.iter(|| {
            // Assuming volume is not required for this calculation.
            let _ = bb.calculate().unwrap();
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_sma(c);
    benchmark_ema(c);
    benchmark_vwma(c);
    benchmark_bbands(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
