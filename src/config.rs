use std::{ops::DerefMut, path::PathBuf, sync::LazyLock};

use bs_cordl::GlobalNamespace::ColorType;
use serde::{Deserialize, Serialize};

use crate::MOD_ID;
use std::sync::Mutex;

// Custom serde implementation for ColorType
mod color_type_serde {
    use bs_cordl::GlobalNamespace::ColorType;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(color: &ColorType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*color as i32)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ColorType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i32::deserialize(deserializer)?;
        Ok(match v {
            0 => ColorType::ColorA,
            1 => ColorType::ColorB,
            -1 => ColorType::None,
            _ => ColorType::default(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[repr(C)]
pub struct Config {
    pub darth_maul_one_hand: bool,
    pub darth_maul_both_hands: bool,
    pub unicorn_mode: bool,
    pub swap_controllers: bool,
    pub one_saber: bool,
    pub one_colour: bool,
    pub swap_top_and_bottom_row: bool,
    pub half_notes: bool,
    pub ignore_burst_sliders: bool,
    pub ignore_arc_sliders: bool,
    #[serde(with = "color_type_serde", default)]
    pub main_hand: ColorType,
    pub disable_rumble: bool,
}

pub static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| {
    let config = Config::read().unwrap_or_default();
    Mutex::new(config)
});

impl Config {
    pub fn config_path() -> PathBuf {
        // https://github.com/QuestPackageManager/beatsaber-hook/blob/cb4d28151b25ac5eda7acc330f4e71e918f8bf71/shared/config/config-utils.hpp#L19-L24
        // /sdcard/ModData/{}/Configs/

        // TODO: Use modloader function to get mod path
        let game_path = "com.beatgames.beatsaber"; // Beatsaber package name
        let mod_id = MOD_ID.to_string_lossy();

        format!("/sdcard/ModData/{game_path}/Configs/{mod_id}.json").into()
    }

    pub fn read() -> anyhow::Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            let default = Self::default();
            default.write()?;
            return Ok(default);
        }

        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let path = Self::config_path();

        let file = std::fs::File::create(&path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            darth_maul_one_hand: false,
            darth_maul_both_hands: false,
            unicorn_mode: false,
            swap_controllers: false,
            one_saber: false,
            one_colour: false,
            swap_top_and_bottom_row: false,
            half_notes: false,
            ignore_burst_sliders: false,
            ignore_arc_sliders: false,
            main_hand: ColorType::ColorA,
            disable_rumble: false,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn darth_maul_get_config() -> Config {
    CONFIG.lock().unwrap().deref_mut().clone()
}

#[unsafe(no_mangle)]
extern "C" fn darth_maul_save_config(config: *mut Config) {
    // let config = CONFIG.lock().unwrap();
    let config = unsafe { &mut *config };
    config.write().expect("Failed to write config");
}
