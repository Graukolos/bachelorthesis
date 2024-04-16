use criterion::{criterion_group, criterion_main, Criterion};
use os_shared::{get_setpoint, iteration, setup};
use std::process::Command;
use std::time::Instant;

pub fn criterion_benchmark(c: &mut Criterion) {
    let output = Command::new("uname").arg("-v").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    if stdout.contains("PREEMPT_RT") {
        panic!()
    }

    let (mut spi, pwm, mut pid) = setup();

    let mut iteration_start = Instant::now();

    c.bench_function("iteration", |b| {
        b.iter(|| {
            iteration_start = iteration(iteration_start, get_setpoint, &mut spi, &mut pid, &pwm);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
