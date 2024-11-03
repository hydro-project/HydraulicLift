use crate::utils::functional::Semigroup;

use super::ir::HOutput;

/// Tracks current node alongside early returns.
pub enum HRail<T> {
    Inner(T),
    Output(HOutput),
    Both(T, HOutput),
}
use HRail::*;

impl<T> HRail<T> {
    pub fn concat_output(self, other: HOutput) -> Self {
        match self {
            Inner(inner) => Both(inner, other),
            Output(output) => Output(output.concat(other)),
            Both(inner, output) => Both(inner, output.concat(other)),
        }
    }
}

/// Monad implementation.
/// This is a special case of the haskell "These" Monad with first generic fixed to HOutput.
/// It is important to note that this only works because HOutput is a Semigroup.
impl<T> HRail<T> {
    /// Monad pure
    pub fn pure(inner: T) -> Self {
        Inner(inner)
    }

    /// Monad bind
    pub fn and_then<U, F>(self, f: F) -> HRail<U>
    where
        F: FnOnce(T) -> HRail<U>,
    {
        match self {
            Inner(inner) => f(inner),
            Output(output) => Output(output),
            Both(inner, output) => f(inner).concat_output(output),
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

impl<T: Semigroup> Semigroup for HRail<T> {
    fn concat(self, other: Self) -> Self {
        match (self, other) {
            (Inner(a), Inner(b)) => Inner(a.concat(b)),
            (Inner(a), Output(y)) => Both(a, y),
            (Inner(a), Both(b, y)) => Both(a.concat(b), y),
            (Output(x), Inner(b)) => Both(b, x),
            (Output(x), Output(y)) => Output(x.concat(y)),
            (Output(x), Both(b, y)) => Both(b, x.concat(y)),
            (Both(a, x), Inner(b)) => Both(a.concat(b), x),
            (Both(a, x), Output(y)) => Both(a, x.concat(y)),
            (Both(a, x), Both(b, y)) => Both(a.concat(b), x.concat(y)),
        }
    }
}

impl HRail<HOutput> {
    pub fn merge(self) -> HOutput {
        match self {
            Inner(a) => a,
            Output(b) => b,
            Both(a, b) => a.concat(b),
        }
    }
}
