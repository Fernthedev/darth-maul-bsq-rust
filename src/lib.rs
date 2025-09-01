#![feature(box_patterns, extend_one)]

use std::ffi::{CStr, CString, c_char};

use quest_hook::libil2cpp::Gc;
use tracing::info;

pub mod config;
pub mod hooks;

unsafe extern "C" {
    unsafe fn doSomething();
}

pub static MOD_ID: &CStr = c"darth_maul";

#[repr(C)]
pub struct ModInfo {
    id: *const c_char,
    version: *const c_char,
    version_long: u64,
}

#[unsafe(no_mangle)]
extern "C" fn setup(modinfo: *mut ModInfo) {
    unsafe {
        *modinfo = ModInfo {
            // we have to let the string leak, because the CString is dropped at the end of the function
            id: MOD_ID.as_ptr(),
            version: CString::new("1.0.0").unwrap().into_raw(),
            version_long: 0,
        }
    }

    // setup quest-hook
    // which will setup tracing and panic logging
    // TODO: Use paper?
    quest_hook::setup("PinkCute");
}

#[unsafe(no_mangle)]
extern "C" fn late_load() {
    info!("Darth Maul mod loaded");

    hooks::install_hooks();
}
