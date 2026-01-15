use std::env;

use std::path::PathBuf;

use quest_build_helper::cc::QuestCpp;
use quest_build_helper::{linker, qpm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let include_dir = manifest_path.join("extern").join("includes");
    let lib_path = manifest_path.join("extern").join("libs");

    // run qpm restore
    qpm::restore(&manifest_path).expect("Failed to restore dependencies");

    linker::setup_linker_defaults();

    // cbindgen::Builder::new()
    //   .with_crate(&manifest_path)
    //   .generate()
    //   .expect("Unable to generate bindings")
    //   .write_to_file("include/bindings.h");

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("wrapper.h")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("quest_compat.rs"))
    //     .expect("Couldn't write bindings!");

    build_cpp(include_dir, lib_path);
    Ok(())
}

fn build_cpp(include_dir: PathBuf, lib_path: PathBuf) {
    // only compile in android linux AARCH64
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let quest = target_os == "android" && target_arch == "aarch64";
    if !quest {
        return;
    }

    linker::linker_flags(lib_path);

    cc::Build::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("cpp/quest_compat.cpp")
        .file("cpp/ModifiersUI.cpp")
        .file("cpp/EnhancedPlayFlowCoordinator.cpp")
        .file("cpp/EnhancedPlaySettingsViewController.cpp")
        // Add quest defaults, defines and includes
        .add_quest_defaults()
        .add_quest_defines()
        .add_il2cpp_includes(&include_dir)
        .add_fmt_includes(&include_dir)
        .add_cordl_includes(&include_dir)
        // we use C++ 20 features
        .flag_if_supported("-std=gnu++20")
        .flag_if_supported("-fPIC")
        .flag_if_supported("-fPIE")
        .flag_if_supported("-frtti")
        .flag_if_supported("-fexceptions")
        .flag_if_supported("-fdeclspec")
        .flag_if_supported("-Wno-invalid-offsetof")
        .define("UNITY_2021", None)
        .define("UNITY_2022", None)
        .define("HAS_CODEGEN", None)
        .define("NEED_UNSAFE_CSHARP", None)
        .define("QUEST", None)
        .define("FMT_HEADER_ONLY", None)
        .include(include_dir)
        .compile("quest_compat");
}
