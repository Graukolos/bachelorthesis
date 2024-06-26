use os_based::{analyze, get_setpoint, iteration, setup};
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    let output = Command::new("uname").arg("-v").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    if stdout.contains("PREEMPT_RT") {
        panic!()
    }

    Command::new("sudo")
        .args([
            "cset",
            "shield",
            "--cpu=3",
            "--kthread=on",
            &format!("--pid={}", std::process::id()),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // setup of communication
    let (mut spi, pwm, mut pid) = setup();
    pid.init(10., 10.);

    let mut iteration_start = Instant::now();

    // warmup
    for _ in 0..10_000 {
        (iteration_start, _) = iteration(iteration_start, get_setpoint, &mut spi, &mut pid, &pwm);
    }

    const N: usize = 100_000;
    let mut times = [Duration::ZERO; N];

    for time in times.iter_mut() {
        (iteration_start, *time) =
            iteration(iteration_start, get_setpoint, &mut spi, &mut pid, &pwm);
    }

    analyze(&times);

    Command::new("sudo")
            .args(["cset", "shield", "--reset"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
}
