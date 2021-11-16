// Disabling the Standard Library
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default.

    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
        }
    }

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("hello again mitron\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "Here are some caluclaitons,
            a = {}, a/b = {}, {}, {}", 3.0/4.0, 1.0/3.0, 1.0/9.0, 7.0/9.0).unwrap();

    print!("aye from hello\t");
    println!("now println says something");
    print!("ahoy, {}\n", "amigo");
    println!("hurray!!! {}", "hello");

    loop {}
}