use crate::utils::functional::Semigroup;

use super::ir::HOutput;

/// Tracks current node alongside early returns
pub enum HRail<T> {
    Inner(T),
    Output(HOutput),
    Both(T, HOutput),
}

impl<T> HRail<T> {
    pub fn concat_output(self, other: HOutput) -> Self {
        match self {
            HRail::Inner(inner) => HRail::Both(inner, other),
            HRail::Output(output) => HRail::Output(output.concat(other)),
            HRail::Both(inner, output) => HRail::Both(inner, output.concat(other)),
        }
    }
}

/// Monad
impl<T> HRail<T> {
    /// Monad pure
    pub fn pure(inner: T) -> Self {
        Self::Inner(inner)
    }

    /// Monad bind
    pub fn and_then<U, F>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> HRail<U>,
    {
        match self {
            HRail::Inner(inner) => f(inner),
            HRail::Output(output) => HRail::Output(output),
            HRail::Both(inner, output) => f(inner).concat_output(output),
        }
    }

    /// Functor map
    pub fn map<F, U>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> U,
    {
        self.and_then(|inner| HRail::pure(f(inner)))
    }
}

impl<T: Semigroup> Semigroup for HRail<T>
{
    fn concat(self, other: Self) -> Self {
        match (self, other) {
            (HRail::Inner(i1), HRail::Inner(i2)) => Self::Inner(i1.concat(i2)),
            (HRail::Inner(i), HRail::Output(o)) => Self::Both(i, o),
            (HRail::Inner(i1), HRail::Both(i2, o)) => Self::Both(i1.concat(i2), o),
            (HRail::Output(o), HRail::Inner(i)) => Self::Both(i, o),
            (HRail::Output(o1), HRail::Output(o2)) => Self::Output(o1.concat(o2)),
            (HRail::Output(o1), HRail::Both(i, o2)) => Self::Both(i, o1.concat(o2)),
            (HRail::Both(i1, o), HRail::Inner(i2)) => Self::Both(i1.concat(i2), o),
            (HRail::Both(i, o1), HRail::Output(o2)) => Self::Both(i, o1.concat(o2)),
            (HRail::Both(i1, o1), HRail::Both(i2, o2)) => Self::Both(i1.concat(i2), o1.concat(o2)),
        }
    }
}