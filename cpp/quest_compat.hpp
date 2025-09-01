// implemented in C++
#include <cstdint>

#define MOD_ID "darth_maul"

// implemented in Rust
extern "C" {

typedef struct Config {
    bool darth_maul_one_hand;
    bool darth_maul_both_hands;
    bool unicorn_mode;
    bool swap_controllers;
    bool one_saber;
    bool one_colour;
    bool swap_top_and_bottom_row;
    bool half_notes;
    bool ignore_burst_sliders;
    bool ignore_arc_sliders;
    // Replace ColorType with the appropriate C-compatible type, e.g., uint32_t or a struct
    uint32_t main_hand; // Example: use uint32_t for color representation
    bool disable_rumble;
} Config;

Config darth_maul_get_config();
void darth_maul_save_config(Config config);
}
