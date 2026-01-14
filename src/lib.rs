#![feature(box_patterns, extend_one)]

use std::ffi::{CStr, CString, c_char};

use bs_cordl::GlobalNamespace;
use quest_hook::libil2cpp::Gc;
use tracing::info;

pub mod config;
pub mod hooks;

unsafe extern "C" {
    fn darth_maul_cpp_init();

    pub fn darth_maul_invoke_GameplaySetupViewController_RefreshContent(
        this: Gc<GlobalNamespace::GameplaySetupViewController>,
    );
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
    quest_hook::setup("DarthMaul");

    use paper2_tracing::init_paper_tracing;
    init_paper_tracing(Some("DarthMaul".to_owned())).expect("Failed to init paper tracing");
}

#[unsafe(no_mangle)]
extern "C" fn late_load() {
    info!("Darth Maul mod loading");

    hooks::install_hooks();

    unsafe { darth_maul_cpp_init() };

    info!("Darth Maul mod finished loading");
}
