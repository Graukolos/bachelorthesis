use clap::Parser;
use clap::Subcommand;
use embedded_hal::spi::SpiBus;
use pid_ctrl::PidCtrl;
use rppal::{
    pwm::{Channel, Polarity, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::time::Instant;
use std::time::Duration;

// TODO:
// perform startup checks on ic-mu
// add multiple setpoint curves

fn main() {
    env_logger::init();

    let args = Args::parse();

    // setup of communication
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 80_000, Mode::Mode0).unwrap();
    let pwm = Pwm::with_frequency(Channel::Pwm0, 1_000., 0.5, Polarity::Normal, false).unwrap();

    let mut pid = PidCtrl::new_with_pid(10., 1., 5.);
    pid.init(10., 10.);

    let mut iteration_start = Instant::now();

    match args.command {
        Commands::Run => {
            loop {
                // fetch step: calculate elapsed time, get new position and setpoint
                let iteration_time = iteration_start.elapsed();
                iteration_start = Instant::now();

                let position = get_position(&mut spi);
                let setpoint = get_setpoint();

                // compute step: calculate new output value
                pid.setpoint = setpoint;
                let output = pid
                    .step(pid_ctrl::PidIn::new(position, iteration_time.as_secs_f64()))
                    .out;

                // update step: output the new value over PWM
                pwm.set_duty_cycle(output).unwrap();
            }
        }
        Commands::Benchmark { iterations } => {
            let mut iteration_times = Vec::with_capacity(iterations as usize);
            for _ in 0..iterations {
                // fetch step: calculate elapsed time, get new position and setpoint
                let iteration_time = iteration_start.elapsed();
                iteration_times.push(iteration_time);
                iteration_start = Instant::now();

                let position = get_position(&mut spi);

                // compute step: calculate new output value
                let output = pid
                    .step(pid_ctrl::PidIn::new(position, iteration_time.as_secs_f64()))
                    .out;

                // update step: output the new value over PWM
                pwm.set_duty_cycle(output).unwrap();
            }

            analyze_iteration_times(iteration_times);
        }
        Commands::Debug => {}
    }
}

fn get_position<Spi: SpiBus>(spi: &mut Spi) -> f64 {
    const SDAD_TRANSMISSION: u8 = 0xa6;
    let mut buf = [SDAD_TRANSMISSION, 0, 0, 0, 0];

    spi.transfer_in_place(&mut buf).unwrap();
    let position = u32::from_be_bytes(buf[1..].try_into().unwrap());

    position as f64
}

fn get_setpoint() -> f64 {
    0.
}

fn analyze_iteration_times(iteration_times: Vec<Duration>) {
    let min = iteration_times.iter().min();
    let max = iteration_times.iter().max();
    let sum: Duration = iteration_times.iter().sum();
    let count = iteration_times.len();
    let avg = sum / count.try_into().unwrap();
    
    dbg!(min);
    dbg!(max);
    dbg!(sum);
    dbg!(count);
    dbg!(avg);
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,
    Benchmark {
        #[arg(default_value_t = 1_000_000)]
        iterations: u64,
    },
    Debug,
}
