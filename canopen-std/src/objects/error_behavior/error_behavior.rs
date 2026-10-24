use super::ErrorBehaviorAction;

/// 0x1029: Error behavior
#[derive(Clone, Debug, Default)]
pub struct ErrorBehavior {
    /// Communication error behavior
    pub communication_error: ErrorBehaviorAction,

    /// Profile- or manufacturer-specific error
    pub manufacture_errors: Vec<ErrorBehaviorAction>,
}

impl ErrorBehavior {
    pub const INDEX: u16 = 0x1029;
    pub const NAME: &'static str = "Error Behavior";
}
