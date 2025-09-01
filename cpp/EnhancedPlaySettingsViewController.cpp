#include "EnhancedPlaySettingsViewController.hpp"
#include "quest_compat.hpp"

#include "bsml/shared/BSML-Lite.hpp"
#include "bsml/shared/BSML.hpp"

DEFINE_TYPE(EnhancedPlay::UI, EnhancedPlaySettingsViewController);

using namespace UnityEngine;
using namespace UnityEngine::UI;
using namespace HMUI;

// simple toggle macro because typing the same thing every time is dumb
#define TOGGLE(name, displayName)                                              \
  CreateToggle(container->get_transform(), displayName, config.name,           \
               [](bool value) -> void {                                        \
                 config.name = value;                                          \
                 SaveConfig();                                                 \
               });

namespace EnhancedPlay::UI {
void EnhancedPlaySettingsViewController::DidActivate(
    bool firstActivation, bool addedToHierarchy, bool screenSystemEnabling) {
  if (firstActivation) {
    GameObject *container =
        BSML::Lite::CreateScrollableSettingsContainer(get_transform());

    /*
    std::vector<StringW> playStyleStringWVector = {
        "Default",
        "Darth Maul One Hand",
        "Darth Maul Two Hands",
        "Unicorn Mode",
        "Swap Controllers",
        "One Saber"
        //"Any Colour"
    };
    StringW playStyleStringW = playStyleStringWVector[config.playStyleMode];

    QuestUI::BeatSaberUI::CreateDropdown(container->get_transform(), "Play
    Style", playStyleStringW, playStyleStringWVector,
        [playStyleStringWVector](std::string value) {
            if (value == playStyleStringWVector[0]) {
                config.playStyleMode = 0;
                enableScoreSubmission();
            }
            else if (value == playStyleStringWVector[1]) {
                config.playStyleMode = 1;
                enableScoreSubmission();
            }
            else if (value == playStyleStringWVector[2]) {
                config.playStyleMode = 2;
                enableScoreSubmission();
            }
            else if (value == playStyleStringWVector[3]) {
                config.playStyleMode = 3;
                disableScoreSubmission();
            }
            //else{
            else if (value == playStyleStringWVector[4]) {
                config.playStyleMode = 4;
                enableScoreSubmission();
            }else{
            //else if (value == playStyleStringWVector[4]) {
                config.playStyleMode = 5;
                disableScoreSubmission();
            }
            //else {
            //    config.playStyleMode = 5;
            //}
            SaveConfig();
        }
    );
    */
    auto config = std::make_shared<Config>(darth_maul_get_config());
    std::vector<std::string_view> mainHandStringWVector = {"Right", "Left"};
    StringW mainHandStringW = mainHandStringWVector[config->main_hand];

    BSML::Lite::CreateDropdown(
        container->get_transform(), "Dominant Hand", mainHandStringW,
        mainHandStringWVector,
        [mainHandStringWVector, config](std::string value) {
          if (value == mainHandStringWVector[0]) {
            config->main_hand = 0;
          } else {
            config->main_hand = 1;
          }
          darth_maul_save_config(*config);
        });

    BSML::Lite::CreateToggle(container->get_transform(), "Swap Controllers",
                             config->swap_controllers, [config](bool value) {
                               config->swap_controllers = value;
                               darth_maul_save_config(*config);
                             });

    BSML::Lite::CreateToggle(container->get_transform(), "Disable Rumble",
                             config->disable_rumble, [config](bool value) {
                               config->disable_rumble = value;
                               darth_maul_save_config(*config);
                             });
  }
}
} // namespace EnhancedPlay::UI