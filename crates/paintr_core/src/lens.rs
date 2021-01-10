use druid::{Lens, LensExt as _};
use std::marker::PhantomData;

pub trait LensMore<A: ?Sized, B: ?Sized>: Lens<A, B> {
    fn tuple<Other, C>(self, other: Other) -> Tuple<Self, Other, B>
    where
        Other: Lens<A, C> + Sized,
        C: ?Sized,
        Self: Sized,
    {
        Tuple::new(self, other)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple<T, U, B: ?Sized> {
    left: T,
    right: U,
    _marker: PhantomData<B>,
}

impl<T, U, B: ?Sized> Tuple<T, U, B> {
    /// Combine two lenses to a Tuple lense
    pub fn new<A: ?Sized, C: ?Sized>(left: T, right: U) -> Self
    where
        T: Lens<A, B>,
        U: Lens<A, C>,
    {
        Self { left, right, _marker: PhantomData }
    }
}

impl<T, U, A, B, C> Lens<A, (B, C)> for Tuple<T, U, B>
where
    A: ?Sized,
    B: Clone,
    C: Clone,
    T: Lens<A, B>,
    U: Lens<A, C>,
{
    fn with<V, F: FnOnce(&(B, C)) -> V>(&self, data: &A, f: F) -> V {
        f(&(self.left.get(data), self.right.get(data)))
    }

    fn with_mut<V, F: FnOnce(&mut (B, C)) -> V>(&self, data: &mut A, f: F) -> V {
        let mut r = (self.left.get(data), self.right.get(data));
        let v = f(&mut r);
        self.left.put(data, r.0);
        self.right.put(data, r.1);
        v
    }
}

impl<A: ?Sized, B: ?Sized, T: Lens<A, B>> LensMore<A, B> for T {}
