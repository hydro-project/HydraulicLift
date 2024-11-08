use super::debug::DebugStr;

/// Wrapping a raw structure with some metadata (in this use case, output scope)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tagged<T, M=()>(pub T, pub M);

impl<T> From<T> for Tagged<T, ()> {
    fn from(value: T) -> Self {
        Self(value, ())
    }
}

impl<T> From<T> for Tagged<DebugStr<T>, ()> {
    fn from(value: T) -> Self {
        Self::from(DebugStr::from(value))
    }
}