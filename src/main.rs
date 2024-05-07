#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os::println;
use rust_os::vga_buffer::{hello_rust_os, WRITER};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hello_rust_os();
    println!("My own Operating System written in Rust");

    #[cfg(test)]
    test_main();

    loop {}
}

// Função Chamada Em Caso De Pânico
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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