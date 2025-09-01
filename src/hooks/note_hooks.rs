use bs_cordl::GlobalNamespace::{
    BeatmapObjectSpawnController, NoteData, NoteData_GameplayType, NoteData_ScoringType,
    NoteLineLayer,
};
use quest_hook::{hook, libil2cpp::Gc};

use crate::config::CONFIG;

#[allow(non_snake_case)]
#[hook("", "BeatmapObjectSpawnController", "HandleNoteDataCallback")]
fn BeatmapObjectSpawnController_HandleNoteDataCallback(
    this: &mut BeatmapObjectSpawnController,
    mut note_data: Gc<NoteData>,
) {
    let config = CONFIG.lock().unwrap();

    if config.one_colour {
        if config.one_saber {
            let to_check_for = config.main_hand;

            if note_data.get_colorType().unwrap() == to_check_for {
                return;
            }
        } else {
            note_data
                .set_colorType(bs_cordl::GlobalNamespace::ColorType::ColorB)
                .expect("Failed to set color type");
        }
    }

    if config.ignore_burst_sliders {
        if note_data.get_gameplayType().unwrap() == NoteData_GameplayType::BurstSliderHead {
            note_data
                .set_gameplayType(bs_cordl::GlobalNamespace::NoteData_GameplayType::Normal)
                .expect("Failed to set gameplay type");
        }

        if note_data.get_scoringType().unwrap() == NoteData_ScoringType::ChainHead
            || note_data.get_scoringType().unwrap() == NoteData_ScoringType::ChainLink
        {
            note_data
                .set_scoringType(NoteData_ScoringType::Normal)
                .expect("Failed to set scoring type");
        }
    }

    if config.ignore_arc_sliders
        && (note_data.get_scoringType().unwrap() == NoteData_ScoringType::ArcTail
            || note_data.get_scoringType().unwrap() == NoteData_ScoringType::ArcHead)
    {
        note_data
            .set_scoringType(NoteData_ScoringType::Normal)
            .expect("Failed to set scoring type");
    }

    if config.swap_top_and_bottom_row {
        let new_layer = match note_data.get_noteLineLayer().unwrap() {
            NoteLineLayer::Base => NoteLineLayer::Top,
            NoteLineLayer::Top => NoteLineLayer::Base,
            other => other,
        };

        note_data
            .set_noteLineLayer(new_layer)
            .expect("Failed to set note line layer");
    }

    if config.half_notes && note_data.get_gameplayType().unwrap() != NoteData_GameplayType::Bomb {
        note_data
            .ChangeToBurstSliderHead()
            .expect("Failed to change to burst slider head");
    }

    BeatmapObjectSpawnController_HandleNoteDataCallback.original(this, note_data);
}

#[allow(non_snake_case)]
#[hook("", "BeatmapObjectSpawnController", "HandleSliderDataCallback")]
fn BeatmapObjectSpawnController_HandleSliderDataCallback(
    this: &mut BeatmapObjectSpawnController,
    mut slider_note_data: Gc<NoteData>,
) {
    let config = CONFIG.lock().unwrap();
    if config.one_colour {
        if config.one_saber {
            let to_check_for = config.main_hand;

            if slider_note_data.get_colorType().unwrap() == to_check_for {
                return;
            }
        } else {
            slider_note_data
                .set_colorType(bs_cordl::GlobalNamespace::ColorType::ColorB)
                .expect("Failed to set color type");
        }
    }
    if config.ignore_burst_sliders {
        if slider_note_data.get_gameplayType().unwrap() == NoteData_GameplayType::BurstSliderHead {
            return;
        }

        if slider_note_data.get_scoringType().unwrap() == NoteData_ScoringType::ChainHead
            || slider_note_data.get_scoringType().unwrap() == NoteData_ScoringType::ChainLink
        {
            return;
        }
    }
    if config.ignore_arc_sliders
        && (slider_note_data.get_scoringType().unwrap() == NoteData_ScoringType::ArcTail
            || slider_note_data.get_scoringType().unwrap() == NoteData_ScoringType::ArcHead)
    {
        return;
    }

    BeatmapObjectSpawnController_HandleSliderDataCallback.original(this, slider_note_data);
}

pub fn install_hooks() {
    BeatmapObjectSpawnController_HandleNoteDataCallback
        .install()
        .unwrap();
    BeatmapObjectSpawnController_HandleSliderDataCallback
        .install()
        .unwrap();
}
