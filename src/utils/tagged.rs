use super::debug::DebugStr;

/// Wrapping a raw structure with some metadata (in this use case, output scope)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagOut<T, M=()>(pub T, pub M);

impl<T> From<T> for TagOut<T, ()> {
    fn from(value: T) -> Self {
        Self(value, ())
    }
}

impl<T> From<T> for TagOut<DebugStr<T>, ()> {
    fn from(value: T) -> Self {
        Self::from(DebugStr::from(value))
    }
}