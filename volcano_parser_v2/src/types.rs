use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn new_from_string(msg: String) -> Self {
        Self { msg }
    }
}

#[macro_export]
macro_rules! parse_error {
    ($x:literal, $($arg:literal),+) => {
        ParseError::new_from_string(format!($x, $($arg),+))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

impl Error for ParseError {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {}

#[derive(Debug, Clone)]
pub struct TokenStream {
    internal: Vec<Token>,
    pos: usize,
}

impl Default for TokenStream {
    fn default() -> Self {
        Self {
            internal: Vec::new(),
            pos: 0,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.internal.len() {
            None
        } else {
            let token = self.internal[self.pos];
            self.pos += 1;
            Some(token)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |count, _| count + 1)
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        self.fold(None, some)
    }

    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        #[inline]
        fn call<T>(mut f: impl FnMut(T)) -> impl FnMut((), T) {
            move |(), item| f(item)
        }

        self.fold((), call(f));
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    fn reduce<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let first = self.next()?;
        Some(self.fold(first, f))
    }

    fn all<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        #[inline]
        fn check<T>(
            mut f: impl FnMut(T) -> bool,
        ) -> impl FnMut((), T) -> std::ops::ControlFlow<()> {
            move |(), x| {
                if f(x) {
                    std::ops::ControlFlow::Continue(())
                } else {
                    std::ops::ControlFlow::Break(())
                }
            }
        }
        self.try_fold((), check(f)) == std::ops::ControlFlow::Continue(())
    }

    fn any<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        #[inline]
        fn check<T>(
            mut f: impl FnMut(T) -> bool,
        ) -> impl FnMut((), T) -> std::ops::ControlFlow<()> {
            move |(), x| {
                if f(x) {
                    std::ops::ControlFlow::Break(())
                } else {
                    std::ops::ControlFlow::Continue(())
                }
            }
        }

        self.try_fold((), check(f)) == std::ops::ControlFlow::Break(())
    }

    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: Ord>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> std::cmp::Ordering {
            x_p.cmp(y_p)
        }

        let (_, x) = self.map(key(f)).max_by(compare)?;
        Some(x)
    }

    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> std::cmp::Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| std::cmp::max_by(x, y, &mut compare)
        }

        self.reduce(fold(compare))
    }

    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: Ord>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> std::cmp::Ordering {
            x_p.cmp(y_p)
        }

        let (_, x) = self.map(key(f)).min_by(compare)?;
        Some(x)
    }

    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> std::cmp::Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| std::cmp::min_by(x, y, &mut compare)
        }

        self.reduce(fold(compare))
    }

    fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        Self: Sized + Iterator<Item = (A, B)>,
    {
        let mut unzipped: (FromA, FromB) = Default::default();
        unzipped.extend(self);
        unzipped
    }

    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: std::iter::Sum<Self::Item>,
    {
        std::iter::Sum::sum(self)
    }

    fn product<P>(self) -> P
    where
        Self: Sized,
        P: std::iter::Product<Self::Item>,
    {
        std::iter::Product::product(self)
    }

    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    {
        !self.eq(other)
    }

    fn lt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less)
    }

    fn le<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    fn gt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater)
    }

    fn ge<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
        Self: Sized,
    {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}
