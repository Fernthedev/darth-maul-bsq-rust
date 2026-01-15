use std::sync::Mutex;

use bs_cordl::{
    GlobalNamespace::{self, StandardLevelGameplayManager_GameState},
    UnityEngine,
};
use quest_hook::{
    hook,
    libil2cpp::{ByRefMut, Gc},
};

use crate::config::CONFIG;

pub static CURRENTLY_IN_LEVEL: Mutex<bool> = Mutex::new(false);
pub static LEFT_HANDED: Mutex<bool> = Mutex::new(false);

#[allow(non_snake_case)]
#[hook("", "StandardLevelGameplayManager", "Update")]
fn StandardLevelGameplayManager_Update(this: &mut GlobalNamespace::StandardLevelGameplayManager) {
    *CURRENTLY_IN_LEVEL.lock().unwrap() =
        this._gameState == StandardLevelGameplayManager_GameState::Playing;
    StandardLevelGameplayManager_Update.original(this);
}

#[allow(non_snake_case)]
#[hook("", "StandardLevelScenesTransitionSetupDataSO", "InitAndSetupScenes")]
fn StandardLevelScenesTransitionSetupDataSO_Init(
    this: &mut GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    playerSpecificSettings: quest_hook::libil2cpp::Gc<GlobalNamespace::PlayerSpecificSettings>,
    backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    startPaused: bool,
) {
    *LEFT_HANDED.lock().unwrap() = playerSpecificSettings._leftHanded;
    StandardLevelScenesTransitionSetupDataSO_Init.original(
        this,
        playerSpecificSettings,
        backButtonText,
        startPaused,
    );
}

// ColorManager_ColorForSaberType
#[allow(non_snake_case)]
#[hook("", "ColorManager", "ColorForSaberType")]
fn ColorManager_ColorForSaberType(
    this: &mut GlobalNamespace::ColorManager,
    type_: GlobalNamespace::SaberType,
) -> UnityEngine::Color {
    if CONFIG.lock().unwrap().one_colour {
        return this._colorScheme._saberBColor.clone();
    }
    ColorManager_ColorForSaberType.original(this, type_)
}

#[allow(non_snake_case)]
#[hook("", "NoteBasicCutInfoHelper", "GetBasicCutInfo")]
fn NoteBasicCutInfoHelper_GetBasicCutInfo(
    noteTransform: Gc<UnityEngine::Transform>,
    colorType: GlobalNamespace::ColorType,
    cutDirection: GlobalNamespace::NoteCutDirection,
    saberType: GlobalNamespace::SaberType,
    saberBladeSpeed: f32,
    cutDirVec: UnityEngine::Vector3,
    cutAngleTolerance: f32,
    directionOK: ByRefMut<bool>,
    speedOK: ByRefMut<bool>,
    mut saberTypeOK: ByRefMut<bool>,
    cutDirDeviation: ByRefMut<f32>,
    cutDirAngle: ByRefMut<f32>,
) {
    NoteBasicCutInfoHelper_GetBasicCutInfo.original(
        noteTransform,
        colorType,
        cutDirection,
        saberType,
        saberBladeSpeed,
        cutDirVec,
        cutAngleTolerance,
        directionOK,
        speedOK,
        saberTypeOK,
        cutDirDeviation,
        cutDirAngle,
    );
    if CONFIG.lock().unwrap().one_colour {
        *saberTypeOK = true;
    }
}

pub fn install_hooks() {
    StandardLevelGameplayManager_Update.install().unwrap();
    StandardLevelScenesTransitionSetupDataSO_Init
        .install()
        .unwrap();
    ColorManager_ColorForSaberType.install().unwrap();
    NoteBasicCutInfoHelper_GetBasicCutInfo.install().unwrap();
}
