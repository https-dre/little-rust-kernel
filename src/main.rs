#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::println;
use rust_os::vga_buffer::hello_rust_os;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hello_rust_os();
    println!("running kernel");
    rust_os::init();
    println!("Initializing Global Descriptor Table and Interrupt Descriptor Table");
    println!("[keyboard suport added]");

    rust_os::hlt_loop();
    
    #[cfg(test)]
    test_main();
}

// Função Chamada Em Caso De Pânico
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}