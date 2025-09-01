use bs_cordl::GlobalNamespace;
use quest_hook::hook;

use crate::darth_maul_invoke_GameplaySetupViewController_RefreshContent;

#[allow(non_snake_case)]
#[hook("", "GameplaySetupViewController", "RefreshContent")]
fn GameplaySetupViewController_RefreshContent(
    this: &mut GlobalNamespace::GameplaySetupViewController,
) {
    // Call the original method first
    unsafe {
        GameplaySetupViewController_RefreshContent.original(this);

        darth_maul_invoke_GameplaySetupViewController_RefreshContent(this.into());
    };
}

pub fn install_hooks() {
    GameplaySetupViewController_RefreshContent
        .install()
        .unwrap();
}
