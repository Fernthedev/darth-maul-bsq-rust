mod modifiers_ui_hook;
mod note_hooks;
mod player_tansform_hooks;
mod rumble_hooks;
mod user_info_hooks;

pub fn install_hooks() {
    note_hooks::install_hooks();
    player_tansform_hooks::install_hooks();
    rumble_hooks::install_hooks();
    user_info_hooks::install_hooks();
    modifiers_ui_hook::install_hooks();
}
