use once_cell::sync::Lazy;

#[derive(Copy, Clone)]
pub enum DebugAst {
    Off,
    On,
    RetainDot,
}

impl DebugAst {
    fn from_env() -> Self {
        if let Ok(string) = std::env::var("DEBUG_AST") {
            if string == "dot" {
                DebugAst::RetainDot
            } else {
                DebugAst::On
            }
        } else {
            DebugAst::Off
        }
    }
}

pub(crate) static DEBUG_AST: Lazy<DebugAst> = Lazy::new(DebugAst::from_env);

pub(crate) static DEBUG_PARSER: Lazy<bool> =
    Lazy::new(|| std::env::var("DEBUG_PARSER").is_ok());
