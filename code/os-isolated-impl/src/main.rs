use embedded_hal::spi::SpiBus;
use pid_ctrl::PidCtrl;
use rppal::{
    pwm::{Channel, Polarity, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::time::Instant;

// TODO:
// perform startup checks on ic-mu
// add multiple setpoint curves

fn main() {
    env_logger::init();

    let args = std::env::args();
    let benchmark = 

    // setup of communication
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 80_000, Mode::Mode0).unwrap();
    let pwm = Pwm::with_frequency(Channel::Pwm0, 1_000., 0.5, Polarity::Normal, false).unwrap();

    let mut pid = PidCtrl::new_with_pid(10., 1., 5.);
    pid.init(10., 10.);

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
        pid.setpoint = setpoint;
        let output = pid
            .step(pid_ctrl::PidIn::new(position, iteration_time.as_secs_f64()))
            .out;

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
