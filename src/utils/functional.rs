
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
pub struct State<'a, S, T>(Box<dyn 'a + FnOnce(S) -> (S, T)>);

/// Simplified state monad
impl<'a, S: 'a, T: 'a> State<'a, S, T> {
    pub fn run(self, state: S) -> (S, T) {
        self.0(state)
    }

    pub fn eval(self, state: S) -> T {
        self.run(state).1
    }

    pub fn exec(self, state: S) -> S {
        self.run(state).0
    }

    pub fn state<F>(f: F) -> Self
    where
        F: 'a + FnOnce(S) -> (S, T),
    {
        Self(Box::new(|s| f(s)))
    }

    pub fn pure(t: T) -> Self {
        Self::state(|s| (s, t))
    }

    pub fn and_then<U, F>(self, f: F) -> State<'a, S, U>
    where
        U: 'a,
        F: 'a + FnOnce(T) -> State<'a, S, U>,
    {
        State::state(|s1| {
            let (s2, t) = self.0(s1);
            f(t).0(s2)
        })
    }

    pub fn and<U: 'a>(self, u: State<'a, S, U>) -> State<'a, S, U> {
        self.and_then(|_| u)
    }

    pub fn map<U, F>(self, f: F) -> State<'a, S, U>
    where
        F: 'a + FnOnce(T) -> U,
        U: 'a,
    {
        self.and_then(|t| State::pure(f(t)))
    }

    pub fn map_const<U>(self, u: U) -> State<'a, S, U>
    where
        U: 'a,
    {
        self.map(|_| u)
    }
}

/// Additional state monad helper function
impl<'a, S: 'a> State<'a, S, ()> {
    pub fn modify<F>(f: F) -> State<'a, S, ()>
    where
        F: 'a + FnOnce(S) -> S,
    {
        Self::state(|s| (f(s), ()))
    }
}

/// State monad helper
impl<'a, S> State<'a, S, S>
where
    S: 'a + Clone,
{
    pub fn get() -> Self {
        Self::state(|s| (s.clone(), s))
    }
}

/// State monad helper
impl<'a, S> State<'a, S, ()>
where
    S: 'a,
{
    pub fn put(s: S) -> Self {
        Self::state(|_| (s, ()))
    }
}

/// MonadZip implementation
impl<'a, S, T> State<'a, S, T>
where
    S: 'a + Semigroup + Clone,
    T: 'a,
{
    pub fn zip<U: 'a>(self, other: State<'a, S, U>) -> State<'a, S, (T, U)> {
        State(Box::new(|s| {
            let (s1, t1) = self.run(s.clone());
            let (s2, t2) = other.run(s);
            (s1.concat(s2), (t1, t2))
        }))
    }
}