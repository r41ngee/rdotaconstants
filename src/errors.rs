#[derive(Debug)]
/// Representing various errors during
/// resolving an ability JSON fields.
pub enum ResolveValueError {
    /// Appears when requested field
    /// was not found.
    KeyNotFound(String),
    MismatchedPattern {
        value: String,
        pattern: &'static str
    },
    /// Appears when field value could not be
    /// parsed into string.
    StringParseFail(String),
    /// Appears when asked field is not
    /// a [`String`] but an another structure.
    DepthQuery,
}