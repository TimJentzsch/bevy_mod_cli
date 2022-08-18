mod wasm;

use self::wasm::add_wasm;

use super::{context::Context, feature::Feature, utils::select_features};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompileFeature {
    FastCompileTimes,
    WasmTarget,
}

impl Feature for CompileFeature {
    /// A [`Vec`] containing all available compile features.
    fn all() -> Vec<Self> {
        vec![CompileFeature::FastCompileTimes, CompileFeature::WasmTarget]
    }

    /// Determines if a feature should be enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}

impl ToString for CompileFeature {
    fn to_string(&self) -> String {
        match self {
            CompileFeature::FastCompileTimes => "Fast compile times".to_string(),
            CompileFeature::WasmTarget => "Target WASM".to_string(),
        }
    }
}

pub fn select_compile_features() -> Vec<CompileFeature> {
    select_features("Which compile features do you want?")
}

pub fn register_compile_features(context: &mut Context) {
    if context
        .compile_features
        .contains(&CompileFeature::WasmTarget)
    {
        add_wasm(context);
    }
}
