//! Hello, Rust!
//! Prints "Hello, Rust! 🦀" to the console, waits for one second then exits.

#![no_main]
#![no_std]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::time::Duration;

use crate::furi::{Stdout, sleep};

mod furi;
mod sys;

#[repr(C)]
pub struct Foo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo<'_>) -> ! {
    let mut stdout = Stdout;

    write!(&mut stdout, "PANIC!\r\n").unwrap();
    stdout.flush().unwrap();

    // FIXME: Reading `panic_info` causes a crash

    // Halt!
    loop {}
}

#[no_mangle]
pub extern "C" fn hello_rust_app(_p: *mut Foo) -> i32 {
    let mut stdout = Stdout;

    write!(&mut stdout, "Hello, Rust! \u{1F980}\r\n").unwrap();
    stdout.flush().unwrap();

    sleep(Duration::from_secs(1));

    0
}
