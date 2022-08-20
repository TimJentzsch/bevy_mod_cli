use dialoguer::console::{style, Style};

use crate::{
    cargo::{cargo_build, cargo_run, ArgBuilder},
    http_server::launch_game,
    rustup::install_target_if_needed,
    wasm_bindgen::{bundle_to_web, create_wasm_folder_if_needed, install_wasm_bindgen_if_needed},
};

use self::cli::RunCommand;

pub mod cli;

pub fn run(args: &RunCommand) {
    let mut cargo_args = ArgBuilder::new();
    let info_style = Style::new().magenta().dim();

    if args.is_wasm {
        // Make sure that all tools are set up correctly

        // `wasm32-unknown-unknown` compilation target
        install_target_if_needed("wasm32-unknown-unknown", true, false)
            .expect("Installation of compilation target `wasm32-unknown-unknown` failed.");
        // `wasm-bindgen-cli` for bundling
        install_wasm_bindgen_if_needed(true, false)
            .expect("Installation of `wasm-bindgen-cli` failed.");
        // `wasm/` target folder
        create_wasm_folder_if_needed(true).expect("Creation of `wasm/` folder failed.");
    }

    // --bin <NAME>
    if let Some(name) = &args.bin {
        cargo_args.add_with_value("--bin", name);
    }

    // --example <NAME>
    if let Some(name) = &args.example {
        cargo_args.add_with_value("--example", name);
    }

    // --release
    if args.is_release {
        cargo_args.add("--release");
    }

    // --wasm / --target <TRIPLE>
    if args.is_wasm {
        // --wasm takes precedence
        cargo_args.add_with_value("--target", "wasm32-unknown-unknown");
    } else if let Some(triple) = &args.target {
        cargo_args.add_with_value("--target", triple);
    }

    // --target-dir <DIRECTORY>
    if let Some(directory) = &args.target_dir {
        cargo_args.add_with_value("--target-dir", directory);
    }

    // --manifest-path <PATH>
    if let Some(path) = &args.manifest_path {
        cargo_args.add_with_value("--manifest-path", path);
    }

    // Enable dynamic linking if not in release mode and not building for WASM
    if !args.is_release && !args.is_wasm {
        cargo_args.add_with_value("--features", "bevy/dynamic");
    }

    if args.is_wasm {
        println!("{}", info_style.apply_to("Building for WASM..."));
        cargo_build(cargo_args);

        println!("{}", info_style.apply_to("Bundling for the web..."));
        bundle_to_web(args.is_release).expect("Failed to bundle for the web");

        println!("{}", info_style.apply_to("Serving on localhost..."));
        println!(
            "Open your game at <{}>.",
            style("http://127.0.0.1:4000").green()
        );
        launch_game().expect("Failed to launch game");
    } else {
        cargo_run(cargo_args);
    }
}
