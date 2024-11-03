
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

impl<T> Monoid for Option<T> 
where T: Semigroup{
    fn empty() -> Self {
        None
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

/// Simplified state monad
pub struct State<S, T>(Box<dyn FnOnce(S) -> (T, S)>);

/// Simplified state monad
impl<S: 'static, T: 'static> State<S, T> {
    pub fn run(self, state: S) -> (T, S) {
        self.0(state)
    }

    pub fn eval(self, state: S) -> T {
        self.run(state).0
    }

    pub fn exec(self, state: S) -> S {
        self.run(state).1
    }

    pub fn state<F>(f: F) -> Self
    where
        F: 'static + FnOnce(S) -> (T, S),
    {
        Self(Box::new(|s| f(s)))
    }

    pub fn pure(t: T) -> Self {
        Self::state(|s| (t, s))
    }

    pub fn and_then<U, F>(self, f: F) -> State<S, U>
    where
        U: 'static,
        F: 'static + FnOnce(T) -> State<S, U>,
    {
        State::state(|s1| {
            let (t, s2) = self.0(s1);
            f(t).0(s2)
        })
    }

    pub fn and<U: 'static>(self, u: State<S, U>) -> State<S, U> {
        self.and_then(|_| u)
    }

    pub fn map<U, F>(self, f: F) -> State<S, U>
    where
        F: 'static + FnOnce(T) -> U,
        U: 'static,
    {
        self.and_then(|t| State::pure(f(t)))
    }
}

/// Additional state monad helper function
impl<S: 'static> State<S, ()> {
    pub fn modify<F>(f: F) -> State<S, ()>
    where
        F: 'static + FnOnce(S) -> S,
    {
        Self::state(|s| ((), f(s)))
    }
}

/// State monad helper
impl<S> State<S, S>
where
    S: 'static + Clone,
{
    pub fn get() -> Self {
        Self::state(|s| (s.clone(), s))
    }
}

/// State monad helper
impl<S> State<S, ()>
where
    S: 'static,
{
    pub fn put(s: S) -> Self {
        Self::state(|_| ((), s))
    }
}

/// MonadZip implementation
impl<S, T> State<S, T>
where
    S: 'static + Semigroup + Clone,
    T: 'static,
{
    pub fn zip<U: 'static>(self, other: State<S, U>) -> State<S, (T, U)> {
        State(Box::new(|s| {
            let (t1, s1) = self.run(s.clone());
            let (t2, s2) = other.run(s);
            ((t1, t2), s1.concat(s2))
        }))
    }
}