use bs_cordl::{
    GlobalNamespace::{self, ColorType, HapticFeedbackManager},
    Libraries::HM::HMLib,
    UnityEngine::XR::{self, XRNode},
};
use quest_hook::{hook, libil2cpp::Gc};

use crate::config::CONFIG;

#[allow(non_snake_case)]
#[hook("", "SaberClashEffect", "LateUpdate")]

fn SaberClashEffect_LateUpdate(this: &mut GlobalNamespace::SaberClashEffect) {
    let config = CONFIG.lock().unwrap();

    if config.unicorn_mode {
        //Unicorn
        return;
    } else if config.one_saber {
        //OneSaber
        return;
    }
    SaberClashEffect_LateUpdate.original(this);
}

#[allow(non_snake_case)]
#[hook("", "HapticFeedbackManager", "PlayHapticFeedback")]
fn HapticFeedbackController_PlayHapticFeedback(
    this: &mut HapticFeedbackManager,
    mut node: XR::XRNode,
    hapticPreset: Gc<HMLib::VR::HapticPresetSO>,
) {
    // ignore other nodes
    if node != XRNode::LeftHand && node != XRNode::RightHand {
        HapticFeedbackController_PlayHapticFeedback.original(this, node, hapticPreset);
        return;
    }

    let currently_in_level = false;
    if !currently_in_level {
        HapticFeedbackController_PlayHapticFeedback.original(this, node, hapticPreset);
        return;
    }

    let config = CONFIG.lock().unwrap();
    if config.disable_rumble {
        return;
    }

    if config.darth_maul_one_hand {
        //OneHandDarthMaul
        let expected_node = if config.main_hand == ColorType::ColorA {
            XRNode::RightHand
        } else {
            XRNode::LeftHand
        };
        node = expected_node;
        HapticFeedbackController_PlayHapticFeedback.original(this, node, hapticPreset);

        return;
    }

    if config.darth_maul_both_hands {
        //TwoHandDarthMaul
        HapticFeedbackController_PlayHapticFeedback.original(this, XRNode::LeftHand, hapticPreset);
        HapticFeedbackController_PlayHapticFeedback.original(this, XRNode::RightHand, hapticPreset);
        //play in both hands
        return;
    }

    if config.unicorn_mode {
        //UnicornMode
        //disable
        return;
    }

    if config.swap_controllers {
        //Swap Controllers
        let swapped_node = if node == XRNode::LeftHand {
            XRNode::RightHand
        } else {
            XRNode::LeftHand
        };
        HapticFeedbackController_PlayHapticFeedback.original(this, swapped_node, hapticPreset);
        return;
    }

    if config.one_saber {
        //SingleSaber
        // Swap node to main hand if needed
        let main_hand_node = if config.main_hand == ColorType::ColorA {
            XRNode::RightHand
        } else {
            XRNode::LeftHand
        };
        node = main_hand_node;

        HapticFeedbackController_PlayHapticFeedback.original(this, node, hapticPreset);

        return;
    }

    HapticFeedbackController_PlayHapticFeedback.original(this, node, hapticPreset);
}

pub fn install_hooks() {
    HapticFeedbackController_PlayHapticFeedback
        .install()
        .expect("Failed to install HapticFeedbackController_PlayHapticFeedback hook");
    SaberClashEffect_LateUpdate
        .install()
        .expect("Failed to install SaberClashEffect_LateUpdate hook");
}