#include "EnhancedPlayFlowCoordinator.hpp"
#include "EnhancedPlaySettingsViewController.hpp"

#include "bsml/shared/BSML-Lite.hpp"
#include "bsml/shared/BSML.hpp"
#include "bsml/shared/Helpers/creation.hpp"

// #include "Utils/UIUtils.hpp"

#include "HMUI/TitleViewController.hpp"
#include "HMUI/ViewController.hpp"

DEFINE_TYPE(EnhancedPlay::UI, EnhancedPlayFlowCoordinator);

using namespace UnityEngine;
using namespace UnityEngine::UI;
using namespace HMUI;

namespace EnhancedPlay::UI {
void EnhancedPlayFlowCoordinator::DidActivate(bool firstActivation,
                                              bool addedToHierarchy,
                                              bool screenSystemEnabling) {
  if (firstActivation) {
    enhancedPlaySettingsViewController =
        reinterpret_cast<HMUI::ViewController *>(
            BSML::Helpers::CreateViewController<
                EnhancedPlaySettingsViewController *>());

    SetTitle(il2cpp_utils::newcsstr("Enhanced Play Settings"),
             ViewController::AnimationType::Out);
    showBackButton = true;

    ProvideInitialViewControllers(enhancedPlaySettingsViewController, nullptr,
                                  nullptr, nullptr, nullptr);
  }
}

void EnhancedPlayFlowCoordinator::BackButtonWasPressed(
    HMUI::ViewController *topViewController) {

  _parentFlowCoordinator->DismissFlowCoordinator(
      this, ViewController::AnimationDirection::Horizontal, nullptr, false);
}
} // namespace EnhancedPlay::UI