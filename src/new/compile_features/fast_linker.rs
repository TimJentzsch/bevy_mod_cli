use crate::new::context::{Context, CreateFile};

use super::CompileFeature;

pub fn add_fast_linker(context: &mut Context) {
    let mut config_toml = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/.cargo/config.toml"
    ))
    .to_string();

    // Shared generics are only available on nightly
    if context
        .compile_features
        .contains(&CompileFeature::NightlyToolchain)
    {
        config_toml = config_toml.replace("{{{share_generics}}}", r#", "-Zshare-generics=y""#);
    } else {
        config_toml = config_toml.replace("{{{share_generics}}}", "");
    }

    context
        .create_files
        .push(CreateFile::new("/.cargo/config.toml", config_toml));
}
