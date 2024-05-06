#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use vga_buffer::hello_rust_os;
    use core::fmt::Write;
    hello_rust_os();
    write!(vga_buffer::WRITER.lock(), "My Own Operating System Written In Rust\n").unwrap();
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}