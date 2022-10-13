// src/main.rs
#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tiny_os::println;

// mod vga_buffer;
// mod serial;


static HELLO: &'static str = r#"
\
 \
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \
"#;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Wrold!\n{}:$", "root");
    
    #[cfg(test)]
    test_main();

    loop{}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    tiny_os::test_panic_handler(_info);
}