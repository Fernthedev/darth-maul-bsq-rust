#include <utility>

#include "metacore/shared/game.hpp"

#include "quest_compat.hpp"

#include "bsml/shared/BSML-Lite/Creation/Layout.hpp"
#include "bsml/shared/BSML-Lite/Creation/Settings.hpp"
#include "bsml/shared/BSML/Components/Settings/IncrementSetting.hpp"

#include "GlobalNamespace/GameplayModifiersPanelController.hpp"
#include "GlobalNamespace/GameplaySetupViewController.hpp"
#include "UnityEngine/Events/UnityAction.hpp"
#include "UnityEngine/Events/UnityAction_1.hpp"
#include "UnityEngine/GameObject.hpp"
#include "UnityEngine/Quaternion.hpp"
#include "UnityEngine/RectOffset.hpp"
#include "UnityEngine/Resources.hpp"
#include "UnityEngine/Transform.hpp"
#include "UnityEngine/UI/HorizontalLayoutGroup.hpp"
#include "UnityEngine/UI/LayoutElement.hpp"
#include "UnityEngine/UI/VerticalLayoutGroup.hpp"
#include "UnityEngine/Vector3.hpp"
#define MakeDelegate(DelegateType, varName) il2cpp_utils::MakeDelegate<DelegateType>(classof(DelegateType), varName))

BSML::IncrementSetting *modifiersIncrementSetting;
UnityEngine::UI::VerticalLayoutGroup *customModifierContainer;

UnityEngine::Transform *defaultModifiersView;

UnityEngine::Color positiveColourValue;

UnityEngine::UI::Toggle *DM1Button;
UnityEngine::UI::Toggle *UCMButton;
UnityEngine::UI::Toggle *OneSaber;
UnityEngine::UI::Toggle *OneColour;
UnityEngine::UI::Toggle *SwapTopBottomRow;
UnityEngine::UI::Toggle *HalfNotes;
UnityEngine::UI::Toggle *IgnoreChains;
UnityEngine::UI::Toggle *IgnoreArcs;

UnityEngine::UI::LayoutElement *
getLayoutElement(UnityEngine::GameObject *object) {
  auto layoutElement = object->GetComponent<UnityEngine::UI::LayoutElement *>();
  if (layoutElement == nullptr) {
    layoutElement = object->AddComponent<UnityEngine::UI::LayoutElement *>();
  }
  return layoutElement;
}

UnityEngine::UI::Toggle *
createEmptyModifierButton(UnityEngine::Transform *transform) {
  UnityEngine::UI::Toggle *modifToggle = BSML::Lite::CreateModifierButton(
      transform, "", false, nullptr, [=](bool val) {});
  modifToggle->m_Interactable = false;
  modifToggle->get_transform()->Find("BG")->get_gameObject()->SetActive(false);
  return modifToggle;
}

UnityEngine::UI::HorizontalLayoutGroup *createHorizontalGroup() {
  auto horzGroup =
      BSML::Lite::CreateHorizontalLayoutGroup(customModifierContainer);
  horzGroup->set_spacing(1.0);
  // horzGroup->m_Rect->set_offsetMax(1.0);
  // horzGroup->get_padding()->set_top(1);
  // horzGroup->get_padding()->set_bottom(0);
  return horzGroup;
}

// called from rust
extern "C" void darth_maul_invoke_GameplaySetupViewController_RefreshContent(
    GlobalNamespace::GameplayModifiersPanelController *self) {

  // positiveColourValue =
  // panel->gameplayModifierToggles[0]->positiveColor;

  if (modifiersIncrementSetting == nullptr) {

    auto panel = self->gameplayModifiersPanelController;

    defaultModifiersView = panel->get_transform()->Find("Modifiers");
    customModifierContainer =
        BSML::Lite::CreateModifierContainer(panel->get_transform());
    customModifierContainer->get_transform()->Translate(0, -0.154, 0);

    getLayoutElement(customModifierContainer->get_gameObject())
        ->set_preferredWidth(98.0);
    customModifierContainer->set_spacing(1.37);

    auto horzGroup1 = createHorizontalGroup();
    auto horzGroup2 = createHorizontalGroup();
    auto horzGroup3 = createHorizontalGroup();
    auto horzGroup4 = createHorizontalGroup();
    auto horzGroup5 = createHorizontalGroup();

    auto config = std::make_shared<Config>(darth_maul_get_config());

    DM1Button = BSML::Lite::CreateModifierButton(
        horzGroup1->get_transform(), "Darth Maul",
        (config->darth_maul_one_hand || config->darth_maul_both_hands),
        [=](bool val) {
          if (val) {
            HMUI::ModalView *DarthMaulModal = BSML::Lite::CreateModal(
                panel->get_transform(),
                [=](auto modal) {
                  UnityEngine::GameObject::Destroy(modal);
                  if (!(config->darth_maul_both_hands ||
                        config->darth_maul_one_hand)) {
                    DM1Button->set_isOn(false);
                  }
                },
                true);
            auto DarthMaulContainer = BSML::Lite::CreateModifierContainer(
                DarthMaulModal->get_transform());
            getLayoutElement(DarthMaulContainer->get_gameObject())
                ->set_preferredWidth(30.0);
            BSML::Lite::CreateModifierButton(
                DarthMaulContainer->get_transform(), "Darth Maul (One Hand)",
                false, [=](bool val) {
                  DarthMaulModal->Hide(true, nullptr);
                  if (UCMButton->m_IsOn) {
                    config.unicorn_mode = false;
                    UCMButton->set_isOn(false);
                  }
                  if (OneSaber->m_IsOn) {
                    config->one_saber = false;
                    OneSaber->set_isOn(false);
                  }
                  config->darth_maul_one_hand = true;

                  darth_maul_save_config(*config);
                  MetaCore::Game::SetScoreSubmission(MOD_ID, true);
                });
            createEmptyModifierButton(DarthMaulContainer->get_transform());
            BSML::Lite::CreateModifierButton(
                DarthMaulContainer->get_transform(), "Darth Maul (Two Hands)",
                false, [=](bool val) {
                  DarthMaulModal->Hide(true, nullptr);
                  if (UCMButton->m_IsOn) {
                    config->unicorn_mode = false;
                    UCMButton->set_isOn(false);
                  }
                  if (OneSaber->m_IsOn) {
                    config->one_saber = false;
                    OneSaber->set_isOn(false);
                  }
                  config->darth_maul_both_hands = true;

                  darth_maul_save_config(*config);
                  MetaCore::Game::SetScoreSubmission(MOD_ID, true);
                });

            DarthMaulModal->Show(true, true, nullptr);

          } else {

            config->darth_maul_both_hands = val;
            config->darth_maul_one_hand = val;
          }
        });

    UCMButton = BSML::Lite::CreateModifierButton(
        horzGroup1->get_transform(),
        "Unicorn Mode<br><color=#f22>Disables Score Submission</color>",
        config->unicorn_mode, [=](bool val) {
          config->unicorn_mode = val;
          if (val) {
            if (DM1Button->m_IsOn) {
              config->darth_maul_one_hand = false;
              DM1Button->set_isOn(false);
            }
            if (OneSaber->m_IsOn) {
              config->one_saber = false;
              OneSaber->set_isOn(false);
            }
            if (!OneColour->m_IsOn) {
              config->one_colour = true;
              OneColour->set_isOn(true);
            }
            darth_maul_save_config(*config);
            MetaCore::Game::SetScoreSubmission(MOD_ID, false);
          }
        });

    OneSaber = BSML::Lite::CreateModifierButton(
        horzGroup1->get_transform(),
        "Single Saber<br><color=#f22>Disables Score Submission</color>",
        config->one_saber, [=](bool val) {
          config->one_saber = val;

          if (val) {
            if (DM1Button->m_IsOn) {
              config->darth_maul_one_hand = false;
              DM1Button->set_isOn(false);
            }
            if (UCMButton->m_IsOn) {
              config->unicorn_mode = false;
              UCMButton->set_isOn(false);
            }
            if (OneColour->m_IsOn) {
              config->one_colour = false;
              OneColour->set_isOn(false);
            }
            darth_maul_save_config(*config);
            MetaCore::Game::SetScoreSubmission(MOD_ID, false);
          }
        });

    OneColour = BSML::Lite::CreateModifierButton(
        horzGroup2->get_transform(),
        "Single Colour<br><color=#f22>Disables Score Submission</color>",
        config->one_colour, [=](bool val) {
          if (!val && config->unicorn_mode) {
            OneColour->set_isOn(true);
            return;
          }

          config->one_colour = val;

          if (val) {
            if (OneSaber->m_IsOn) {
              config->one_saber = false;
              OneSaber->set_isOn(false);
            }
          }
          darth_maul_save_config(*config);
          MetaCore::Game::SetScoreSubmission(MOD_ID, false);
        });

    SwapTopBottomRow = BSML::Lite::CreateModifierButton(
        horzGroup2->get_transform(),
        "TECH<br><color=#f22>Disables Score Submission</color>",
        config->swap_top_and_bottom_row, [=](bool val) {
          if (!IgnoreChains->m_IsOn) {
            config->ignore_burst_sliders = true;
            IgnoreChains->set_isOn(true);
          }
          if (!IgnoreArcs->m_IsOn) {
            config->ignore_burst_sliders = true;
            IgnoreArcs->set_isOn(true);
          }

          config->swap_top_and_bottom_row = val;

          darth_maul_save_config(*config);
          MetaCore::Game::SetScoreSubmission(MOD_ID, false);
        });

    HalfNotes = BSML::Lite::CreateModifierButton(
        horzGroup2->get_transform(),
        "Half Notes<br><color=#f22>Disables Score Submission</color>",
        config->half_notes, [=](bool val) {
          config->half_notes = val;

          darth_maul_save_config(*config);
          MetaCore::Game::SetScoreSubmission(MOD_ID, false);
        });

    IgnoreChains = BSML::Lite::CreateModifierButton(
        horzGroup3->get_transform(),
        "Ignore Chains<br><color=#f22>Disables Score Submission</color>",
        config->ignore_burst_sliders, [=](bool val) {
          if (!val && config->swap_top_and_bottom_row) {
            IgnoreChains->set_isOn(true);
            return;
          }

          config->ignore_burst_sliders = val;

          darth_maul_save_config(*config);
          MetaCore::Game::SetScoreSubmission(MOD_ID, false);
        });

    IgnoreArcs = BSML::Lite::CreateModifierButton(
        horzGroup3->get_transform(),
        "Ignore Arcs<br><color=#f22>Disables Score Submission</color>",
        config->ignore_arc_sliders, [=](bool val) {
          if (!val && config->swap_top_and_bottom_row) {
            IgnoreArcs->set_isOn(true);
            return;
          }

          config->ignore_arc_sliders = val;

          darth_maul_save_config(*config);
          MetaCore::Game::SetScoreSubmission(MOD_ID, false);
        });

    createEmptyModifierButton(horzGroup3->get_transform());

    createEmptyModifierButton(horzGroup4->get_transform());

    createEmptyModifierButton(horzGroup5->get_transform());

    modifiersIncrementSetting = BSML::Lite::CreateIncrementSetting(
        panel->get_transform(), "Select Modifiers", 0, 1.0, 0.0, true, true,
        0.0, 1.0, {0, 0}, [=](float val) {
          if (val == 0.0) {
            modifiersIncrementSetting->set_text("Default Modifiers");
            defaultModifiersView->get_gameObject()->SetActive(true);

            customModifierContainer->get_gameObject()->SetActive(false);

          } else if (val == 1.0) {
            modifiersIncrementSetting->set_text("Custom Modifiers");
            defaultModifiersView->get_gameObject()->SetActive(false);

            customModifierContainer->get_gameObject()->SetActive(true);
          }
        });
    customModifierContainer->get_gameObject()->SetActive(false);

    modifiersIncrementSetting->get_transform()->Translate({0, -1.9, 0});
    modifiersIncrementSetting->set_text("Default Modifiers");
  }
}
