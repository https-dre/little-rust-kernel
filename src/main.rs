#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use vga_buffer::hello_rust_os;
    use core::fmt::Write;
    hello_rust_os();
    write!(vga_buffer::WRITER.lock(), "My Own Operating System Written In Rust\n").unwrap();
    println!("Hello World!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}