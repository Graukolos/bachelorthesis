use std::time::Instant;

use embedded_hal::spi::SpiBus;
use pid::Pid;
use rppal::{
    pwm::{Channel, Polarity, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
};

// TODO:
// tune kp, ki and kd
// perform startup checks on ic-mu
// add multiple setpoint curves

fn main() {
    env_logger::init();

    // setup of communication
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 80_000, Mode::Mode0).unwrap();
    let pwm = Pwm::with_frequency(Channel::Pwm0, 1_000., 0.5, Polarity::Normal, false).unwrap();

    let mut pid = Pid::new(10., 100.);
    pid.p(10., 100.);
    pid.i(1., 100.);
    pid.d(5., 100.);

    let start = Instant::now();
    let mut iteration_start = Instant::now();

    loop {
        // fetch step: calculate elapsed time, get new position and setpoint
        let iteration_time = iteration_start.elapsed();
        log::debug!(
            "iteration_time in milliseconds: {}",
            iteration_time.as_millis()
        );
        iteration_start = Instant::now();

        let position = get_position(&mut spi);
        let setpoint = get_setpoint(&start);

        log::debug!("current position: {}", position);
        log::debug!("current setpoint: {}", setpoint);

        // compute step: calculate new output value
        pid.setpoint(setpoint);
        let output = pid.next_control_output(position).output;

        // update step: output the new value over PWM
        pwm.set_duty_cycle(output).unwrap();
    }
}

fn get_position<Spi: SpiBus>(spi: &mut Spi) -> f64 {
    const SDAD_TRANSMISSION: u8 = 0xa6;
    let mut buf = [SDAD_TRANSMISSION, 0, 0, 0, 0];

    spi.transfer_in_place(&mut buf).unwrap();
    let position = u32::from_be_bytes(buf[1..].try_into().unwrap());

    position as f64
}

fn get_setpoint(start: &Instant) -> f64 {
    start.elapsed().as_secs_f64() % 100.
}
