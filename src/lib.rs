#![feature(box_patterns, extend_one)]

use scotland2_rs::{ModInfo, scotland2_raw::CModInfo};
use tracing::info;

use crate::config::Config;

pub mod config;
pub mod hooks;

#[cfg(feature = "ui")]
#[link(name = "quest_compat", kind = "static")]
unsafe extern "C" {
    fn darth_maul_cpp_init();

    pub fn darth_maul_invoke_GameplaySetupViewController_RefreshContent(
        this: Gc<GlobalNamespace::GameplaySetupViewController>,
    );
}

pub static MOD_ID: &str = "darth_maul";

#[unsafe(no_mangle)]
extern "C" fn setup(modinfo: &mut CModInfo) {
    *modinfo = ModInfo {
        // we have to let the string leak, because the CString is dropped at the end of the function
        id: MOD_ID.to_string(),
        version: "1.0.0".to_owned(),
        version_long: 1,
    }
    .into();

    // setup quest-hook
    // which will setup tracing and panic logging
    // quest_hook::setup("DarthMaul");

    #[cfg(all(target_os = "android", feature = "paper2"))]
    {
        use paper2_tracing::init_paper_tracing;
        init_paper_tracing(Some("DarthMaul".to_owned())).expect("Failed to init paper tracing");
    }

    std::panic::set_hook(quest_hook::panic_hook(true, true));
}

#[unsafe(no_mangle)]
extern "C" fn late_load() {
    info!("Darth Maul mod loading");

    info!("Load config from disk {}", Config::config_path().display());
    let _config = config::CONFIG.lock().unwrap();

    info!("Installing hooks");

    hooks::install_hooks();

    #[cfg(feature = "ui")]
    unsafe {
        darth_maul_cpp_init()
    };

    info!("Darth Maul mod finished loading");
}
