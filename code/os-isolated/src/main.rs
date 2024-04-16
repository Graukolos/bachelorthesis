use os_shared::{get_setpoint, iteration, setup};
use std::process::Command;
use std::time::Instant;

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

    ctrlc::set_handler(|| {
        Command::new("sudo")
            .args(["cset", "shield", "--reset"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    })
    .unwrap();

    // setup of communication
    let (mut spi, pwm, mut pid) = setup();
    pid.init(10., 10.);

    let mut iteration_start = Instant::now();

    loop {
        iteration_start = iteration(iteration_start, get_setpoint, &mut spi, &mut pid, &pwm);
    }
}
