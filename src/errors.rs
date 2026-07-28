#[derive(Debug)]
pub enum ResolveValueError {
    KeyNotFound(String),
    MismatchedPattern {
        value: String,
        pattern: &'static str
    },
    StringParseFail(String),
    DepthQuery,
}