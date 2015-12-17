//! This module contains Purescript-inspired effects monads for rust
//!
//! Here, an effect is defined an evaluatable function.
#![feature(fn_traits, unboxed_closures)]

macro_rules! effect_map {
    ( $e:expr ) => {
        move || $e
    };
    ( $b:block ) => {
        move || $b
    };
}

/// Helper enum for acting as a resolve function.
///
/// Ideally, we would use a closure instead of this type, but this type exists
/// as a workaround alternative to avoid using boxed closures
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

impl<T> From<T> for ResolveFn<T> {
    fn from(v: T) -> Self {
        use ResolveFn::Const;
        Const(v)
    }
}

/// Monad trait for effect functions
pub trait EffectMonad<A>: Sized {
    /// Sequentially composes two effect functions, passing
    /// the output of the first to the input of the second
    fn bind<B, Eb, F>(self, f: F) -> BoundEffect<Self, F>
        where Eb: FnOnce() -> B,
              F: FnOnce(A) -> Eb;

    /// Sequentially composes the two effects, while ignoring the return values
    /// of the effects. Similar to the `>>` function in Haskell, but without
    /// returning the value of the second Monad.
    ///
    /// Shorthand for
    /// ```rust
    /// effectMonad.bind(|_| someOtherEffectMonad);
    /// ```
    #[inline(always)]
    fn bind_ignore_contents<B, Eb>(self, eb: Eb) -> BoundEffect<Self, ResolveFn<Eb>>
        where Eb: FnOnce() -> B,
    {
        self.bind(eb.into())
    }
}

impl<T, A> EffectMonad<A> for T
    where T: FnOnce() -> A,
{
    #[inline(always)]
    fn bind<B, Eb, F>(self, f: F) -> BoundEffect<Self, F>
        where Eb: FnOnce() -> B,
              F: FnOnce(A) -> Eb,
    {
        bind_effects(self, f)
    }
}

/// A struct representing two bound effects. Ideally, we would be able to a
/// closure here, but that's not possible without returning a boxed version of
/// the closure, which we don't want to do.
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

    #[test]
    fn println_can_be_mapped_as_effect() {
        effect_map!(println!("hello")).bind_ignore_contents(effect_map!(println!("goodbye")))();
    }

    #[test]
    fn effect_map_performs_effect() {
        let mut x: isize = 0;
        {
            let px = &mut x;
            effect_map!(*px += 1)();
        }
        assert_eq!(x, 1);
    }

    #[test]
    fn effect_can_implicitly_borrow() {
        let mut x = 1;
        {
            (|| {
                x += 5;
            })();
        }
        assert_eq!(x, 6);
    }

    #[test]
    fn effect_map_compiles_block() {
        let mut x: isize =  0;
        {
            let px = &mut x;
            effect_map!({
                *px = 42;
            })()
        }
        assert_eq!(x, 42);
    }

    #[test]
    fn effect_monad_bind_safely_chains_state() {
        let mut x: isize = 0;
        {
            let px = &mut x;
            (effect_map!({
                *px = 6;
                px
            })).bind(|px| effect_map!(*px += 1))();
        }
        assert_eq!(x, 7);
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
