# Minimal Operating System Kernel

A kernel written in Rust, whose development goal is nothing more and nothing less than to acquire knowledge on the subject.

It's a simple project, not complete compared to Linux, Windows or MacOs. The project was created with the aim of learning and creating materials on the subject in Portuguese.

## Implementations

The project is still in the early stages of development, some kernel subsystems have not been added.

There is no version available for use, as the kernel is small and does not yet have some of the features required to run as an operating system. It is not recommended to try to boot a real machine using the image generated by the cargo.

- VGA Text Mode [x]

- CPU Exceptions [x]

- Keyboard ASCII [x]

- MM - *Memory Manager (in progress)*

- PM - *Process Manager*

- IPC - *Interprocess Communication*

- VFS - *Virtual File System*

## Running with QEMU

In order to emulate and run the project, you need to download the source code and compile it using cargo (Rust's dependency manager) to generate a "bootable" image.

To run the project, you need to have the entire setup with Rust nightly configured.

    rustup default nightly

Install bootimage (lib responsible for generating a bootloader):

    cargo install bootimage

Download the source code:

    git clone https://github.com/https-dre/little-rust-kernel
    cd little-rust-kernel

Run the project using qemu (make sure you have qemu installed):

    cargo run

To run with integrated tests:

    cargo test [test_name]