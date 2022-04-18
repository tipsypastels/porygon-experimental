/// Returned by `Scope::try_skip()` to determine if a setup step instance
/// should be skipped. Could be a `bool`, but I immediately mixed up the
/// meanings when I did that.
#[derive(Debug, Clone, Copy)]
pub enum Skip {
    Skip,
    Proceed,
}

impl Skip {
    pub fn should_skip(self) -> bool {
        matches!(self, Skip::Skip)
    }
}
