use bs_cordl::{
    GlobalNamespace::{self, ColorType},
    UnityEngine::{self, Vector3},
};
use quest_hook::hook;

use crate::config::CONFIG;

#[allow(non_snake_case)]
#[hook("", "PlayerTransforms", "Update")]
fn PlayerTransforms_Update(this: &mut GlobalNamespace::PlayerTransforms) {
    let config = CONFIG.lock().unwrap();

    // Early return if either hand transform is missing
    if this._leftHandTransform.is_null() || this._rightHandTransform.is_null() {
        return;
    }

    // OneHandDarthMaul
    if config.darth_maul_one_hand {
        let mut main_saber = this._rightHandTransform;
        let mut saber_to_move = this._leftHandTransform;
        if config.main_hand == ColorType::ColorB {
            std::mem::swap(&mut main_saber, &mut saber_to_move);
        }

        let main_saber_rot = main_saber.get_rotation().expect("Failed to get rotation");
        let main_saber_pos = main_saber.get_position().expect("Failed to get position");

        saber_to_move
            .set_rotation(main_saber_rot)
            .expect("Failed to set rotation");
        saber_to_move
            .Rotate_Vector3_1(UnityEngine::Vector3 {
                x: 0.0,
                y: 180.0,
                z: 0.0,
            })
            .expect("Failed to rotate");
        saber_to_move
            .set_position(main_saber_pos)
            .expect("Failed to set position");
        saber_to_move
            .Translate_Vector3_1(UnityEngine::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.26,
            })
            .expect("Failed to translate");
        return;
    }

    // TwoHandDarthMaul
    if config.darth_maul_both_hands {
        let mut left = this._leftHandTransform;
        let mut right = this._rightHandTransform;

        let left_pos = left.get_position().expect("Failed to get position");
        let right_pos = right.get_position().expect("Failed to get position");
        let mid_pos = Vector3 {
            x: (left_pos.x + right_pos.x) / 2.0,
            y: (left_pos.y + right_pos.y) / 2.0,
            z: (left_pos.z + right_pos.z) / 2.0,
        };
        right.set_position(mid_pos).expect("Failed to set position");
        right.LookAt_Vector3_3(left_pos).expect("Failed to look at");
        right
            .Translate_Vector3_1(UnityEngine::Vector3 {
                x: 0.0,
                y: 0.0,
                z: -0.13,
            })
            .expect("Failed to translate");

        left.set_position(right.get_position().expect("Failed to get position"))
            .expect("Failed to set position");
        left.set_rotation(right.get_rotation().expect("Failed to get rotation"))
            .expect("Failed to set rotation");
        right
            .Rotate_Vector3_1(UnityEngine::Vector3 {
                x: 0.0,
                y: 180.0,
                z: 0.0,
            })
            .expect("Failed to rotate");
        left.Translate_Vector3_1(UnityEngine::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.26,
        })
        .expect("Failed to translate");
        return;
    }

    // UnicornMode
    if config.unicorn_mode {
        let mut right = this._rightHandTransform;
        let mut left = this._leftHandTransform;

        right.set_rotation(this._headWorldRot.clone()).expect("Failed to set rotation");
        right.set_position(this._headWorldPos.clone()).expect("Failed to set position");
        left.set_position(UnityEngine::Vector3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        }).expect("Failed to set position");
        return;
    }

    // OneSaber
    if config.one_saber {
        // TODO: Get left handed from game settings
        let left_handed = false;
        let swap_controllers = config.swap_controllers;
        let mut left = this._leftHandTransform;
        let mut right = this._rightHandTransform;
        let offscreen = UnityEngine::Vector3 { x: 0.0, y: -1000.0, z: 0.0 };

        match (left_handed, swap_controllers) {
            (false, false) => {
                left.set_position(offscreen).expect("Failed to set position");
            }
            (false, true) => {
                right.set_position(left.get_position().expect("Failed to get position")).expect("Failed to set position");
                right.set_rotation(left.get_rotation().expect("Failed to get rotation")).expect("Failed to set rotation");
                left.set_position(offscreen).expect("Failed to set position");
            }
            (true, false) => {
                right.set_position(offscreen).expect("Failed to set position");
            }
            (true, true) => {
                left.set_position(right.get_position().expect("Failed to get position")).expect("Failed to set position");
                left.set_rotation(right.get_rotation().expect("Failed to get rotation")).expect("Failed to set rotation");
                right.set_position(offscreen).expect("Failed to set position");
            }
        }
        return;
    }

    // Swap Controllers
    if config.swap_controllers {
        let mut left = this._leftHandTransform;
        let mut right = this._rightHandTransform;

        let left_rot = left.get_rotation().expect("Failed to get rotation");
        let left_pos = left.get_position().expect("Failed to get position");

        left.set_position(right.get_position().expect("Failed to get position")).expect("Failed to set position");
        left.set_rotation(right.get_rotation().expect("Failed to get rotation")).expect("Failed to set rotation");

        right.set_position(left_pos).expect("Failed to set position");
        right.set_rotation(left_rot).expect("Failed to set rotation");
        return;
    }
}

pub fn install_hooks() {
    PlayerTransforms_Update.install().expect("Failed to install PlayerTransforms_Update hook");
}

// MAKE_AUTO_HOOK_MATCH(PlayerTransforms_Update, &GlobalNamespace::PlayerTransforms::Update, void, GlobalNamespace::PlayerTransforms* self)
// {
// 	if (self->leftHandTransform == nullptr || self->rightHandTransform == nullptr) {
// 		PlayerTransforms_Update(self);
// 		return;
// 	}

// 	if (config.darthMaulOneHand) {
// 		//OneHandDarthMaul
// 		UnityEngine::Transform* mainSaber;
// 		UnityEngine::Transform* saberToMove;

// 		if (config.mainHand == 0) {
// 			//right handed
// 			mainSaber = self->rightHandTransform;
// 			saberToMove = self->leftHandTransform;
// 		}
// 		else {
// 			//left handed
// 			mainSaber = self->leftHandTransform;
// 			saberToMove = self->rightHandTransform;
// 		}

// 		UnityEngine::Quaternion mainSaberRot = mainSaber->get_rotation();
// 		UnityEngine::Vector3 mainSaberPos = mainSaber->get_position();
// 		saberToMove->set_rotation(mainSaberRot);
// 		saberToMove->Rotate({ 0,180,0 });
// 		saberToMove->set_position(mainSaberPos);
// 		saberToMove->Translate({ 0, 0, 0.26 });

// 		PlayerTransforms_Update(self);
// 		return;
// 	}
// 	else if (config.darthMaulBothHands) {
// 		//TwoHandDarthMaul

// 		UnityEngine::Vector3 rightSaberPos = self->leftHandTransform->get_position();
// 		self->rightHandTransform->set_position({ ((self->leftHandTransform->get_position().x + self->rightHandTransform->get_position().x) / 2), ((self->leftHandTransform->get_position().y + self->rightHandTransform->get_position().y) / 2), ((self->leftHandTransform->get_position().z + self->rightHandTransform->get_position().z) / 2) });
// 		self->rightHandTransform->LookAt(rightSaberPos);
// 		self->rightHandTransform->Translate({ 0, 0, -0.13 });

// 		self->leftHandTransform->set_position(self->rightHandTransform->get_position());
// 		self->leftHandTransform->set_rotation(self->rightHandTransform->get_rotation());
// 		self->rightHandTransform->Rotate({ 0,180,0 });
// 		self->leftHandTransform->Translate({ 0, 0, 0.26 });

// 		PlayerTransforms_Update(self);
// 		return;
// 	}
// 	else if (config.unicornMode) {
// 		//UnicornMode

// 		self->rightHandTransform->set_rotation(self->headWorldRot);
// 		self->rightHandTransform->set_position(self->headWorldPos);
// 		self->leftHandTransform->set_position({0,-1000,0});
// 		PlayerTransforms_Update(self);
// 		return;
// 	}

// 	else if (config.oneSaber) {
// 		//OneSaber

// 		if(!leftHanded){
// 			//normal mode
// 			if (!config.swapControllers) {
// 				//right handed
// 				self->leftHandTransform->set_position({0,-1000,0});
// 			}
// 			else {
// 				//left handed
// 				self->rightHandTransform->set_position(self->leftHandTransform->get_position());
// 				self->rightHandTransform->set_rotation(self->leftHandTransform->get_rotation());
// 				self->leftHandTransform->set_position({0,-1000,0});

// 			}

// 		}else{
// 			//left handed mode
// 			if (!config.swapControllers) {
// 				//right handed
// 				self->rightHandTransform->set_position({0,-1000,0});
// 			}
// 			else {
// 				//left handed
// 				self->leftHandTransform->set_position(self->rightHandTransform->get_position());
// 				self->leftHandTransform->set_rotation(self->rightHandTransform->get_rotation());
// 				self->rightHandTransform->set_position({0,-1000,0});
// 			}
// 		}

// 		PlayerTransforms_Update(self);
// 		return;
// 	}
// 	else if (config.swapControllers) {
// 		//Swap Controllers
// 		UnityEngine::Quaternion leftSaberRot = self->leftHandTransform->get_rotation();
// 		UnityEngine::Vector3 leftSaberPos = self->leftHandTransform->get_position();

// 		self->leftHandTransform->set_position(self->rightHandTransform->get_position());
// 		self->leftHandTransform->set_rotation(self->rightHandTransform->get_rotation());

// 		self->rightHandTransform->set_position(leftSaberPos);
// 		self->rightHandTransform->set_rotation(leftSaberRot);

// 		PlayerTransforms_Update(self);
// 		return;
// 	}
// 	PlayerTransforms_Update(self);
// }
