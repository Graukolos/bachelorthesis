use core::marker::PhantomData;

pub trait GpioExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

pub struct Unknown;

pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

pub struct Floating;
pub struct PullDown;
pub struct PullUp;

pub struct Output;

// GPFselx, gpsetx, gpclrx
macro_rules! gpio {
    ([$($PXi:ident: (
        $pxi:ident,
        $gpfselx:ident,
        $fselx:ident,
        $gplevx:ident,
        $levx:ident,
        $gpio_pup_pdn_cntrl_regx:ident,
        $gpio_pup_pdn_cntrlx:ident,
        $gpsetx:ident,
        $setx:ident,
        $gpclrx:ident,
        $clrx:ident,
        $MODE:ty
    ),)+]) => {
        use bcm2711_lpa::GPIO;
        use embedded_hal as hal;
        use core::convert::Infallible;

        pub struct Parts {
            $(
                pub $pxi: $PXi<$MODE>,
            )+
        }

        impl GpioExt for GPIO {
            type Parts = Parts;

            fn split(self) -> Parts {
                Parts {
                    $(
                        $pxi: $PXi { _mode: PhantomData },
                    )+
                }
            }
        }

        $(
            pub struct $PXi<MODE> {
                _mode: PhantomData<MODE>,
            }

            impl<MODE> $PXi<MODE> {
                pub fn into_input(self) -> $PXi<Input<Floating>> {
                    unsafe { (*GPIO::PTR).$gpio_pup_pdn_cntrl_regx().set_bits(|w| {w.$gpio_pup_pdn_cntrlx().none()})}

                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().input()}) };

                    $PXi { _mode: PhantomData }
                }

                pub fn into_input_pulldown(self) -> $PXi<Input<PullDown>> {
                    unsafe { (*GPIO::PTR).$gpio_pup_pdn_cntrl_regx().set_bits(|w| {w.$gpio_pup_pdn_cntrlx().down()})}

                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().input()}) };

                    $PXi { _mode: PhantomData }
                }

                pub fn into_input_pullup(self) -> $PXi<Input<PullUp>> {
                    unsafe { (*GPIO::PTR).$gpio_pup_pdn_cntrl_regx().set_bits(|w| {w.$gpio_pup_pdn_cntrlx().up()})}

                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().input()}) };

                    $PXi { _mode: PhantomData }
                }

                pub fn into_output(self) -> $PXi<Output> {
                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().output()})}

                    $PXi { _mode: PhantomData }
                }

                pub fn into_output_low(self) -> $PXi<Output> {
                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().output()})}

                    $PXi { _mode: PhantomData }
                }

                pub fn into_output_high(self) -> $PXi<Output> {
                    unsafe { (*GPIO::PTR).$gpfselx().set_bits(|w| {w.$fselx().output()})}

                    $PXi { _mode: PhantomData }
                }
            }

            impl<MODE> hal::digital::InputPin for $PXi<Input<MODE>> {
                fn is_high(&mut self) -> Result<bool, Self::Error>{
                    Ok(unsafe{ (*GPIO::PTR).$gplevx().read().$levx().bit_is_set() })
                }

                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(unsafe{ (*GPIO::PTR).$gplevx().read().$levx().bit_is_clear() })
                }
            }

            impl<MODE> hal::digital::ErrorType for $PXi<MODE> {
                type Error = Infallible;
            }

            impl hal::digital::OutputPin for $PXi<Output> {
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    Ok(unsafe{ (*GPIO::PTR).$gpsetx().write_with_zero(|w| {w.$setx().set_bit()}) })
                }

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    Ok(unsafe{ (*GPIO::PTR).$gpclrx().write_with_zero(|w| {w.$clrx().clear_bit_by_one()}) })
                }
            }
        )+
    }
}

gpio!([
    Pin0: (pin0, gpfsel0, fsel0, gplev0, lev0, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl0, gpset0, set0, gpclr0, clr0, Unknown),
    Pin1: (pin1, gpfsel0, fsel1, gplev0, lev1, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl1, gpset0, set1, gpclr0, clr1, Unknown),
    Pin2: (pin2, gpfsel0, fsel2, gplev0, lev2, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl2, gpset0, set2, gpclr0, clr2, Unknown),
    Pin3: (pin3, gpfsel0, fsel3, gplev0, lev3, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl3, gpset0, set3, gpclr0, clr3, Unknown),
    Pin4: (pin4, gpfsel0, fsel4, gplev0, lev4, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl4, gpset0, set4, gpclr0, clr4, Unknown),
    Pin5: (pin5, gpfsel0, fsel5, gplev0, lev5, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl5, gpset0, set5, gpclr0, clr5, Unknown),
    Pin6: (pin6, gpfsel0, fsel6, gplev0, lev6, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl6, gpset0, set6, gpclr0, clr6, Unknown),
    Pin7: (pin7, gpfsel0, fsel7, gplev0, lev7, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl7, gpset0, set7, gpclr0, clr7, Unknown),
    Pin8: (pin8, gpfsel0, fsel8, gplev0, lev8, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl8, gpset0, set8, gpclr0, clr8, Unknown),
    Pin9: (pin9, gpfsel0, fsel9, gplev0, lev9, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl9, gpset0, set9, gpclr0, clr9, Unknown),
    Pin10: (pin10, gpfsel1, fsel10, gplev0, lev10, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl10, gpset0, set10, gpclr0, clr10, Unknown),
    Pin11: (pin11, gpfsel1, fsel11, gplev0, lev11, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl11, gpset0, set11, gpclr0, clr11, Unknown),
    Pin12: (pin12, gpfsel1, fsel12, gplev0, lev12, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl12, gpset0, set12, gpclr0, clr12, Unknown),
    Pin13: (pin13, gpfsel1, fsel13, gplev0, lev13, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl13, gpset0, set13, gpclr0, clr13, Unknown),
    Pin14: (pin14, gpfsel1, fsel14, gplev0, lev14, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl14, gpset0, set14, gpclr0, clr14, Unknown),
    Pin15: (pin15, gpfsel1, fsel15, gplev0, lev15, gpio_pup_pdn_cntrl_reg0, gpio_pup_pdn_cntrl15, gpset0, set15, gpclr0, clr15, Unknown),
    Pin16: (pin16, gpfsel1, fsel16, gplev0, lev16, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl16, gpset0, set16, gpclr0, clr16, Unknown),
    Pin17: (pin17, gpfsel1, fsel17, gplev0, lev17, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl17, gpset0, set17, gpclr0, clr17, Unknown),
    Pin18: (pin18, gpfsel1, fsel18, gplev0, lev18, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl18, gpset0, set18, gpclr0, clr18, Unknown),
    Pin19: (pin19, gpfsel1, fsel19, gplev0, lev19, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl19, gpset0, set19, gpclr0, clr19, Unknown),
    Pin20: (pin20, gpfsel2, fsel20, gplev0, lev20, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl20, gpset0, set20, gpclr0, clr20, Unknown),
    Pin21: (pin21, gpfsel2, fsel21, gplev0, lev21, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl21, gpset0, set21, gpclr0, clr21, Unknown),
    Pin22: (pin22, gpfsel2, fsel22, gplev0, lev22, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl22, gpset0, set22, gpclr0, clr22, Unknown),
    Pin23: (pin23, gpfsel2, fsel23, gplev0, lev23, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl23, gpset0, set23, gpclr0, clr23, Unknown),
    Pin24: (pin24, gpfsel2, fsel24, gplev0, lev24, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl24, gpset0, set24, gpclr0, clr24, Unknown),
    Pin25: (pin25, gpfsel2, fsel25, gplev0, lev25, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl25, gpset0, set25, gpclr0, clr25, Unknown),
    Pin26: (pin26, gpfsel2, fsel26, gplev0, lev26, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl26, gpset0, set26, gpclr0, clr26, Unknown),
    Pin27: (pin27, gpfsel2, fsel27, gplev0, lev27, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl27, gpset0, set27, gpclr0, clr27, Unknown),
    Pin28: (pin28, gpfsel2, fsel28, gplev0, lev28, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl28, gpset0, set28, gpclr0, clr28, Unknown),
    Pin29: (pin29, gpfsel2, fsel29, gplev0, lev29, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl29, gpset0, set29, gpclr0, clr29, Unknown),
    Pin30: (pin30, gpfsel3, fsel30, gplev0, lev30, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl30, gpset0, set30, gpclr0, clr30, Unknown),
    Pin31: (pin31, gpfsel3, fsel31, gplev0, lev31, gpio_pup_pdn_cntrl_reg1, gpio_pup_pdn_cntrl31, gpset0, set31, gpclr0, clr31, Unknown),
    Pin32: (pin32, gpfsel3, fsel32, gplev1, lev32, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl32, gpset1, set32, gpclr1, clr32, Unknown),
    Pin33: (pin33, gpfsel3, fsel33, gplev1, lev33, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl33, gpset1, set33, gpclr1, clr33, Unknown),
    Pin34: (pin34, gpfsel3, fsel34, gplev1, lev34, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl34, gpset1, set34, gpclr1, clr34, Unknown),
    Pin35: (pin35, gpfsel3, fsel35, gplev1, lev35, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl35, gpset1, set35, gpclr1, clr35, Unknown),
    Pin36: (pin36, gpfsel3, fsel36, gplev1, lev36, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl36, gpset1, set36, gpclr1, clr36, Unknown),
    Pin37: (pin37, gpfsel3, fsel37, gplev1, lev37, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl37, gpset1, set37, gpclr1, clr37, Unknown),
    Pin38: (pin38, gpfsel3, fsel38, gplev1, lev38, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl38, gpset1, set38, gpclr1, clr38, Unknown),
    Pin39: (pin39, gpfsel3, fsel39, gplev1, lev39, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl39, gpset1, set39, gpclr1, clr39, Unknown),
    Pin40: (pin40, gpfsel4, fsel40, gplev1, lev40, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl40, gpset1, set40, gpclr1, clr40, Unknown),
    Pin41: (pin41, gpfsel4, fsel41, gplev1, lev41, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl41, gpset1, set41, gpclr1, clr41, Unknown),
    Pin42: (pin42, gpfsel4, fsel42, gplev1, lev42, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl42, gpset1, set42, gpclr1, clr42, Unknown),
    Pin43: (pin43, gpfsel4, fsel43, gplev1, lev43, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl43, gpset1, set43, gpclr1, clr43, Unknown),
    Pin44: (pin44, gpfsel4, fsel44, gplev1, lev44, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl44, gpset1, set44, gpclr1, clr44, Unknown),
    Pin45: (pin45, gpfsel4, fsel45, gplev1, lev45, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl45, gpset1, set45, gpclr1, clr45, Unknown),
    Pin46: (pin46, gpfsel4, fsel46, gplev1, lev46, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl46, gpset1, set46, gpclr1, clr46, Unknown),
    Pin47: (pin47, gpfsel4, fsel47, gplev1, lev47, gpio_pup_pdn_cntrl_reg2, gpio_pup_pdn_cntrl47, gpset1, set47, gpclr1, clr47, Unknown),
    Pin48: (pin48, gpfsel4, fsel48, gplev1, lev48, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl48, gpset1, set48, gpclr1, clr48, Unknown),
    Pin49: (pin49, gpfsel4, fsel49, gplev1, lev49, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl49, gpset1, set49, gpclr1, clr49, Unknown),
    Pin50: (pin50, gpfsel5, fsel50, gplev1, lev50, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl50, gpset1, set50, gpclr1, clr50, Unknown),
    Pin51: (pin51, gpfsel5, fsel51, gplev1, lev51, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl51, gpset1, set51, gpclr1, clr51, Unknown),
    Pin52: (pin52, gpfsel5, fsel52, gplev1, lev52, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl52, gpset1, set52, gpclr1, clr52, Unknown),
    Pin53: (pin53, gpfsel5, fsel53, gplev1, lev53, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl53, gpset1, set53, gpclr1, clr53, Unknown),
    Pin54: (pin54, gpfsel5, fsel54, gplev1, lev54, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl54, gpset1, set54, gpclr1, clr54, Unknown),
    Pin55: (pin55, gpfsel5, fsel55, gplev1, lev55, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl55, gpset1, set55, gpclr1, clr55, Unknown),
    Pin56: (pin56, gpfsel5, fsel56, gplev1, lev56, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl56, gpset1, set56, gpclr1, clr56, Unknown),
    Pin57: (pin57, gpfsel5, fsel57, gplev1, lev57, gpio_pup_pdn_cntrl_reg3, gpio_pup_pdn_cntrl57, gpset1, set57, gpclr1, clr57, Unknown),
]);
