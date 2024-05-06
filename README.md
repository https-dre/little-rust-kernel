# My own operating system written in Rust

To compile the system, use the command below:
  
    cargo build --target rust_os.json

Use the command below to run with qemu:

    qemu-system-x86_64 -drive format=raw,file=target/dev_os/debug/bootimage-dev_os.bin
