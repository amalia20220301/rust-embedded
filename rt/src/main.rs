#![allow(unused)]
#![no_main]
#![no_std]

fn main() {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_panic: &PanicInfo<'_>) -> ! {
        loop {}
    }
}
