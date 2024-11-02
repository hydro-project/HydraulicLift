
pub trait Semigroup {
    fn concat(self, other: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

impl<T> Semigroup for Option<T>
where
    T: Semigroup,
{
    fn concat(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.concat(t2)),
        }
    }
}

// TODO: Implement unified fake functor/applicative/monad

/// No higher kinded types :(
pub trait FakeFunctor {
    type T;
    type With<U>;
    fn map<F, U>(self, f: F) -> Self::With<U> where F: FnOnce(Self::T) -> U ;
}

impl<X, T> FakeFunctor for (X, T) {
    type T=T;
    type With<U> = (X, U);
    fn map<F, U>(self, f: F) -> Self::With<U> where F: FnOnce(Self::T) -> U {
        (self.0, f(self.1))
    }
}