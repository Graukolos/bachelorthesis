use embedded_hal::spi::SpiBus;
use ic_mu::ICMU;
use pid::Pid;
use rppal::{pwm::{Channel, Polarity, Pwm}, spi::{Bus, Mode, SimpleHalSpiDevice, SlaveSelect, Spi}};



fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 80_000, Mode::Mode0).unwrap();
    let mut pwm = Pwm::with_frequency(Channel::Pwm0, 1_000., 0.5, Polarity::Normal, false).unwrap();
    
    let mut pid = Pid::new(10., 100.);
    pid.p(10., 100.);
    pid.i(1., 100.);
    pid.d(5., 100.);

    loop {
        let position = get_position(&mut spi);
        pwm.set_duty_cycle(pid.next_control_output(position).output).unwrap();
    }
}

fn get_position<Spi: SpiBus>(spi: &mut Spi) -> f64 {
    const SDAD_TRANSMISSION: u8 = 0xa6;
    let mut buf = [SDAD_TRANSMISSION, 0, 0, 0, 0];

    spi.transfer_in_place(&mut buf).unwrap();
    let mut position_bytes = [0; 4];
    position_bytes.copy_from_slice(&buf[1..]);
    let position = u32::from_be_bytes(position_bytes);
    position as f64
}

fn get_setpoint() -> f64 {
    
    10.
}