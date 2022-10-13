/*!
 * tests/should_panic.rs
 * 
 * A significant drawback of this approach is that it only works for a single test function. 
 * With multiple #[test_case] functions, only the first function is executed because the execution 
 * cannot continue after the panic handler has been called. 
 * 
 * I currently don’t know of a good way to solve this problem, so let me know if you have an idea!
 * 
 * With no harness tests, we can test case in _start instead of test_runner
 */
#![no_std]
#![no_main]

// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// 
// #![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use tiny_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test_main();

    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}


// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     exit_qemu(QemuExitCode::Success);
// }

// #[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}