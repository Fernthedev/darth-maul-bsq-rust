#include "metacore/shared/game.hpp"

#include "custom-types/shared/register.hpp"

#include "bsml/shared/BSML.hpp"

#include "EnhancedPlayFlowCoordinator.hpp"
#include "EnhancedPlaySettingsViewController.hpp"

#include "quest_compat.hpp"

extern "C" {
void darth_maul_set_score_submission(bool enabled) {
  MetaCore::Game ::SetScoreSubmission("darth_maul", enabled);
}

bool darth_maul_get_score_submission() {
  return MetaCore::Game::IsScoreSubmissionDisabled();
}

void darth_maul_cpp_init() {
  custom_types::Register::AutoRegister();

  BSML::Register::RegisterSettingsMenu<
      EnhancedPlay::UI::EnhancedPlayFlowCoordinator *>("Enhanced Play");
}
}
