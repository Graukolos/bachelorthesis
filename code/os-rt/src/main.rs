use os_shared::{get_setpoint, iteration, setup};

use std::{process::Command, time::Instant};

fn main() {
    let output = Command::new("uname").arg("-v").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    if stdout.contains("PREEMPT_RT") {
        panic!()
    }

    // setup of communication
    let (mut spi, pwm, mut pid) = setup();
    pid.init(10., 10.);

    let mut iteration_start = Instant::now();

    loop {
        iteration_start = iteration(iteration_start, get_setpoint, &mut spi, &mut pid, &pwm);
    }
}
