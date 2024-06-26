#![no_std]
#![no_main]

extern crate alloc;
use alloc::{format, string::String, vec};
use core::{
    alloc::GlobalAlloc,
    ffi::{c_int, c_uint, c_void},
    panic::PanicInfo,
    ptr::null_mut,
};
use pid_ctrl::PidCtrl;

mod ffi;

// just reboot on rust panic
#[panic_handler]
unsafe fn panic_handler(_panic_info: &PanicInfo) -> ! {
    ffi::reboot()
}

// use circle's allocator implementation for rust
struct CircleAllocator;
unsafe impl GlobalAlloc for CircleAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        ffi::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        ffi::free(ptr as *mut c_void)
    }
}

#[global_allocator]
static ALLOCATOR: CircleAllocator = CircleAllocator;

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut act_led = ffi::CActLED::new(false);
    let options = ffi::CKernelOptions::new();
    let mut device_name_service = ffi::CDeviceNameService::new();
    let mut screen = ffi::CScreenDevice::new(options.GetWidth(), options.GetHeight(), false, 0);
    let mut interrupt_system = ffi::CInterruptSystem::new();
    let mut serial = ffi::CSerialDevice::new(&mut interrupt_system, false, 0);
    let _exception_handler = ffi::CExceptionHandler::new();
    let mut timer = ffi::CTimer::new(&mut interrupt_system);
    let mut logger = ffi::CLogger::new(options.GetLogLevel(), &mut timer, true);
    let mut usb_hci =
        ffi::CXHCIDevice::new(&mut interrupt_system, &mut timer, false, 0, null_mut());
    let mut filesystem = ffi::CFATFileSystem::new();

    screen.Initialize();
    serial.Initialize(115200, 8, 1, ffi::CSerialDevice_TParity_ParityNone);
    let mut log_device = device_name_service.GetDevice(options.GetLogDevice(), false);
    if log_device.is_null() {
        log_device = &mut serial._base;
    }
    logger.Initialize(log_device);
    interrupt_system.Initialize();
    timer.Initialize();
    ((*usb_hci._base._base.vtable_).CUSBController_Initialize)(&mut usb_hci._base._base, true);

    const SPI_FREQ: c_uint = 20_000_000;
    const CPOL: c_uint = 0;
    const CPHA: c_uint = 0;
    const SPI_DEVICE: c_uint = 0;
    let mut spi = ffi::CSPIMaster::new(SPI_FREQ, CPOL, CPHA, SPI_DEVICE);
    spi.Initialize();

    let _pwm_pin = ffi::CGPIOPin::new1(
        18,
        ffi::TGPIOMode_GPIOModeAlternateFunction5,
        core::ptr::null_mut(),
    );
    let _pwm_pin_19 = ffi::CGPIOPin::new1(
        19,
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

    act_led.Blink(5, 500, 300);

    pwm.Start();

    let mut iteration_start = ffi::CTimer::GetClockTicks64();

    #[cfg(feature = "measure")]
    {
        for _ in 0..10_000 {
            let position = get_position(&mut spi);
            let setpoint = get_setpoint();

            pid.setpoint = setpoint;
            let time = ffi::CTimer::GetClockTicks64() - iteration_start;
            let output = pid
                .step(pid_ctrl::PidIn::new(
                    position,
                    f64::from(time as u32) / 1_000_000.,
                ))
                .out;
            iteration_start = ffi::CTimer::GetClockTicks64();

            pwm.Write(
                ffi::PWM_CHANNEL1 as c_uint,
                ((output * PWM_RANGE as f64).clamp(0., PWM_RANGE as f64)) as c_uint,
            );
        }
        const N: usize = 1_000_000;
        let mut times = vec![0; N];
        for time in times.iter_mut() {
            let position = get_position(&mut spi);
            let setpoint = get_setpoint();

            pid.setpoint = setpoint;
            *time = ffi::CTimer::GetClockTicks64() - iteration_start;
            let output = pid
                .step(pid_ctrl::PidIn::new(
                    position,
                    f64::from(*time as u32) / 1_000_000.,
                ))
                .out;
            iteration_start = ffi::CTimer::GetClockTicks64();

            pwm.Write(
                ffi::PWM_CHANNEL1 as c_uint,
                ((output * PWM_RANGE as f64).clamp(0., PWM_RANGE as f64)) as c_uint,
            );
        }

        let partition = device_name_service.GetDevice(c"umsd1-1".as_ptr(), true);
        filesystem.Mount(partition);

        let file = filesystem.FileCreate(c"times.csv".as_ptr());
        let mut buffer = String::from("iteration,elapsed_time_us\n");
        for (n, time) in times[1..].iter().enumerate() {
            buffer.push_str(&format!("{},{}\n", n, time));
        }
        let buffer = alloc::ffi::CString::new(buffer).unwrap();

        filesystem.FileWrite(
            file,
            buffer.as_ptr() as *const c_void,
            buffer.count_bytes() as u32,
        );
        filesystem.FileClose(file);
        filesystem.UnMount();

        ffi::halt()
    }

    #[cfg(not(feature = "measure"))]
    loop {
        let position = get_position(&mut spi);
        let setpoint = get_setpoint();

        pid.setpoint = setpoint;
        let time = ffi::CTimer::GetClockTicks64() - iteration_start;
        let output = pid
            .step(pid_ctrl::PidIn::new(
                position,
                f64::from(time as u32) / 1_000_000.,
            ))
            .out;
        iteration_start = ffi::CTimer::GetClockTicks64();

        pwm.Write(
            PWM_CHANNEL1 as c_uint,
            ((output * PWM_RANGE as f64).clamp(0., PWM_RANGE as f64)) as c_uint,
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
