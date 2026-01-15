use bs_cordl::GlobalNamespace;
use quest_hook::hook;
use tracing::info;

use crate::config::{CONFIG, Config};

#[allow(non_snake_case)]
#[hook("", "GameplaySetupViewController", "RefreshContent")]
fn GameplaySetupViewController_RefreshContent(
    this: &mut GlobalNamespace::GameplaySetupViewController,
) {
    // Call the original method first
    GameplaySetupViewController_RefreshContent.original(this);

    *CONFIG.lock().unwrap() = Config::read().unwrap();

    info!("Darth maul config: {:#?}", CONFIG.lock().unwrap());

    #[cfg(feature = "ui")]
    unsafe {
        crate::darth_maul_invoke_GameplaySetupViewController_RefreshContent(this.into());
    };
}

pub fn install_hooks() {
    GameplaySetupViewController_RefreshContent
        .install()
        .unwrap();
}
