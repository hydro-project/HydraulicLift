pub trait Semigroup {
    fn concat(self, other: Self) -> Self;
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

// [m a] -> m [a]
pub trait SequenceState<'a, S, T>: IntoIterator<Item = State<'a, S, T>> {
    fn sequence(self) -> State<'a, S, Vec<T>>;
}

impl<'a, U, S, T> SequenceState<'a, S, T> for U
where
    U: IntoIterator<Item = State<'a, S, T>>,
    S: 'a,
    T: 'a,
{
    fn sequence(self) -> State<'a, S, Vec<T>> {
        self.into_iter().fold(State::pure(Vec::new()), |acc, x| {
            acc.and_then(|mut v| {
                x.map(|x| {
                    v.push(x);
                    v
                })
            })
        })
    }
}
