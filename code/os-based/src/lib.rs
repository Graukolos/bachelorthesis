use std::time::{Duration, Instant};

use embedded_hal::spi::SpiBus;
use pid_ctrl::PidCtrl;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

pub fn iteration<F: Fn() -> f64>(
    last_iteration_start: Instant,
    get_setpoint: F,
    spi: &mut Spi,
    pid: &mut PidCtrl<f64>,
    pwm: &Pwm,
) -> (Instant, Duration) {
    // fetch step: calculate elapsed time, get new position and setpoint
    let iteration_time = last_iteration_start.elapsed();
    let iteration_start = Instant::now();

    let position = get_position(spi);
    let setpoint = get_setpoint();

    // compute step: calculate new output value
    pid.setpoint = setpoint;
    let output = pid
        .step(pid_ctrl::PidIn::new(position, iteration_time.as_secs_f64()))
        .out;

    // update step: output the new value over PWM
    pwm.set_duty_cycle(output).unwrap();

    (iteration_start, iteration_time)
}

pub fn setup() -> (Spi, Pwm, PidCtrl<f64>) {
    (
        Spi::new(Bus::Spi0, SlaveSelect::Ss0, 20_000_000, Mode::Mode0).unwrap(),
        Pwm::with_frequency(Channel::Pwm0, 1_000., 0.5, Polarity::Normal, false).unwrap(),
        PidCtrl::new_with_pid(10., 1., 5.),
    )
}

fn get_position(spi: &mut Spi) -> f64 {
    const SDAD_TRANSMISSION: u8 = 0xa6;
    let mut buf = [SDAD_TRANSMISSION, 0, 0, 0, 0];

    spi.transfer_in_place(&mut buf).unwrap();
    let position = u32::from_be_bytes(buf[1..].try_into().unwrap());

    position as f64
}

pub fn get_setpoint() -> f64 {
    0.
}
