#![no_main]

extern crate winapi;
extern crate user32;

use std::ffi::OsStr;
use std::iter::once;
use std::os::raw::{c_char, c_int, c_void};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(_ : *const c_void, _ : *const c_void, _ : *const c_char, _ : c_int) -> c_int {
    let title = text("Rust Test with winapi");
    let message = text("Hello, winapi! (MessageBox)");

    unsafe {
        user32::MessageBoxW(null_mut(), message.as_ptr(), title.as_ptr(), winapi::MB_OK);
    }

    0

}

fn text(text: &str) -> Vec<u16> {
    return OsStr::new(text).encode_wide().chain(once(0)).collect::<Vec<u16>>();
}


// fn main() {
//     println!("Hello, world!");
// }
