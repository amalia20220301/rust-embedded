#![allow(unused)]
#![no_main]
#![no_std]

#![allow(unused)]
#![feature(lang_items)]


fn main() {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_panic: &PanicInfo<'_>) -> ! {
        loop {}
    }

    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}
}
