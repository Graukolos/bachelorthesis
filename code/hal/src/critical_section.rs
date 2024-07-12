use aarch64_cpu::registers::Writeable;
use aarch64_cpu::registers::DAIF;
use critical_section::RawRestoreState;

struct SingleCoreCS;
critical_section::set_impl!(SingleCoreCS);

unsafe impl critical_section::Impl for SingleCoreCS {
    unsafe fn acquire() -> RawRestoreState {
        DAIF.write(DAIF::A::Unmasked + DAIF::I::Unmasked + DAIF::F::Unmasked);
    }

    unsafe fn release(_: RawRestoreState) {
        DAIF.write(DAIF::A::Masked + DAIF::I::Masked + DAIF::F::Masked);
    }
}
