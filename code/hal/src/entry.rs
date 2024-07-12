use core::arch::asm;

use aarch64_cpu::{
    asm::wfe,
    registers::{Readable, MPIDR_EL1, MPIDR_EL1::Aff0},
};

#[no_mangle]
pub unsafe extern "C" fn __start() -> ! {
    if MPIDR_EL1.read(Aff0) == 0 {
        // safe because MPIDR_EL1.read() already checks if we're on aarch64
        asm!("bl __start_rust");
    }

    loop {
        wfe()
    }
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn __start_rust() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}
