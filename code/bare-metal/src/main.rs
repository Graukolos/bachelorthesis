#![no_std]
#![no_main]

use core::{
    ffi::{c_int, c_uint, c_void, CStr},
    panic::PanicInfo,
};
use ffi::PWM_CHANNEL1;
use pid_ctrl::PidCtrl;

mod ffi;

#[panic_handler]
unsafe fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

const FROM_KERNEL: &CStr = c"kernel";

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut act_led = ffi::CActLED::new(false);
    let options = ffi::CKernelOptions::new();
    let mut device_name_service = ffi::CDeviceNameService::new();
    let mut serial = ffi::CSerialDevice::new(core::ptr::null_mut(), false, 0);
    let _exception_handler = ffi::CExceptionHandler::new();
    let mut interrupt_system = ffi::CInterruptSystem::new();
    let mut screen = ffi::CScreenDevice::new(options.GetWidth(), options.GetHeight(), false, 0);
    let mut timer = ffi::CTimer::new(&mut interrupt_system);
    let mut logger = ffi::CLogger::new(options.GetLogLevel(), &mut timer, true);

    act_led.Blink(3, 200, 500);

    screen.Initialize();
    act_led.Blink(3, 200, 500);
    serial.Initialize(115200, 8, 1, ffi::CSerialDevice_TParity_ParityNone);
    act_led.Blink(3, 200, 500);
    logger.Initialize(match device_name_service.GetDevice(options.GetLogDevice(), false) {
        core::ptr::null_mut() as *mut ffi::CDevice => &mut screen._base,
        c_device => c_device
    });
    logger.Initialize(&mut screen._base);
    act_led.Blink(3, 200, 500);
    interrupt_system.Initialize();
    act_led.Blink(3, 200, 500);
    timer.Initialize();
    act_led.Blink(3, 200, 500);

    const SPI_FREQ: c_uint = 115200;
    const CPOL: c_uint = 0;
    const CPHA: c_uint = 0;
    const SPI_DEVICE: c_uint = 0;
    let mut spi = ffi::CSPIMaster::new(SPI_FREQ, CPOL, CPHA, SPI_DEVICE);
    spi.Initialize();

    const N_PIN: u32 = 18;
    let _pwm_pin = ffi::CGPIOPin::new1(
        N_PIN,
        ffi::TGPIOMode_GPIOModeAlternateFunction5,
        core::ptr::null_mut(),
    );
    const PWM_RANGE: u32 = 1024;
    let mut pwm = ffi::CPWMOutput::new(
        ffi::TGPIOClockSource_GPIOClockSourceOscillator,
        2,
        PWM_RANGE,
        true,
    );

    let mut pid = PidCtrl::new_with_pid(10., 1., 5.);

    let mut iteration_start;

    loop {
        iteration_start = ffi::CTimer::GetClockTicks64();

        let position = get_position(&mut spi);
        let setpoint = get_setpoint();

        pid.setpoint = setpoint;
        let output = pid.step(pid_ctrl::PidIn::new(position, 0.)).out;

        pwm.Write(
            PWM_CHANNEL1 as c_uint,
            ((output * PWM_RANGE as f64).clamp(0., PWM_RANGE as f64)) as c_uint,
        );

        let mut message = { ffi::CString::new() };

        ffi::CString_Format(
            &mut message,
            c"Microseconds since iteration start: %u".as_ptr(),
            ffi::CTimer::GetClockTicks64() - iteration_start,
        );

        ffi::CLogger_Write(
            &mut logger,
            FROM_KERNEL.as_ptr(),
            ffi::TLogSeverity_LogNotice,
            message.m_pBuffer,
        );
    }
}

unsafe fn get_position(spi: &mut ffi::CSPIMaster) -> f64 {
    const SDAD_TRANSMISSION: u8 = 0xa6;
    let buf_write = [SDAD_TRANSMISSION, 0, 0, 0, 0];
    let mut buf_read = [0; 5];

    const CS: u32 = 0;

    spi.WriteRead(
        CS,
        buf_write.as_ptr() as *const c_void,
        buf_read.as_mut_ptr() as *mut c_void,
        5,
    );

    let position = u32::from_be_bytes(buf_read[1..].try_into().unwrap());

    position as f64
}

pub fn get_setpoint() -> f64 {
    0.
}
