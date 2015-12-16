//! This module contains Purescript-inspired effects monads for rust
#![feature(fn_traits, unboxed_closures)]

/// Helper enum for acting as a resolve function
pub enum ResolveFn<T> {
    Const(T),
}

impl<T, Args> FnOnce<Args> for ResolveFn<T> {
    type Output = T;

    #[inline(always)]
    extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
        use ResolveFn::Const;

        match self {
            Const(v) => v,
        }
    }
}

/// Monad trait for effect functions
pub trait EffectMonad<A>: Sized {
    /// Sequentially composes two effect functions, passing
    /// the output of the first to the input of the second
    fn bind<B, Eb, F>(self, f: F) -> BoundEffect<Self, F>
        where Eb: FnOnce() -> B,
              F: FnOnce(A) -> Eb;
    fn bind_ignore_contents<B, Eb>(self, eb: Eb) -> BoundEffect<Self, ResolveFn<Eb>>
        where Eb: FnOnce() -> B,
    {
        self.bind(ResolveFn::Const(eb))
    }
}

impl<T, A> EffectMonad<A> for T
    where T: FnOnce() -> A,
{
    fn bind<B, Eb, F>(self, f: F) -> BoundEffect<Self, F>
        where Eb: FnOnce() -> B,
              F: FnOnce(A) -> Eb,
    {
        bind_effects(self, f)
    }
}

/// A struct representing two bound effects
pub struct BoundEffect<Ea, F> {
    ea: Ea,
    f: F,
}

impl<A, B, Ea, Eb, F> FnOnce<()> for BoundEffect<Ea, F>
    where Ea: FnOnce() -> A,
          Eb: FnOnce() -> B,
          F: FnOnce(A) -> Eb,
{
    type Output = B;
    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        // This is for readability... hopefully it'll be optimized out
        let a_result = (self.ea)();
        (self.f)(a_result)()
    }
}

fn bind_effects<A, B, Ea, Eb, F>(first: Ea, f: F) -> BoundEffect<Ea, F>
    where Ea: FnOnce() -> A,
          Eb: FnOnce() -> B,
          F: FnOnce(A) -> Eb,
{
    BoundEffect {
        ea: first,
        f: f,
    }
}

#[cfg(test)]
mod public_test {
    use super::*;

    #[test]
    fn effect_monad_bind_performs() {
        let mut x: isize = 0;
        let px = &mut x as *mut isize;
        (|| unsafe {
            *px += 2;
        }).bind_ignore_contents(|| unsafe {
            *px -= 1;
        })();
        assert_eq!(x, 1);
    }

    #[test]
    fn effect_monad_bind_performs_sequentially() {
        let mut x: isize = 3;
        let px = &mut x as *mut isize;
        (|| unsafe {
            *px *= 2;
        }).bind_ignore_contents(|| unsafe {
            *px -= 1;
        })();
        assert_eq!(x, 5);
    }

    #[test]
    fn effect_monad_bind_binds() {
        let mut x: isize = 0;
        let px = &mut x as *mut isize;
        (|| unsafe {
            *px *= 2;
            42
        }).bind(|a: isize| {
            move || unsafe {
                *px = a
            }
        })();
        assert_eq!(x, 42);
    }
}

// It's OK for the code in the following tests to be "unsafe" becuase we know
// that only one of the closures will ever be executing at once.
//
// The REAL way to implement this pattern would be with a state monad rather
// than an effect monad

#[test]
fn bind_effect_binds() {
    let mut x: isize = 0;
    let px = &mut x as *mut isize;
    bind_effects(|| unsafe {
        *px *= 2;
        42
    }, |a: isize| {
        move || unsafe {
            *px = a
        }
    })();
    assert_eq!(x, 42);
}
