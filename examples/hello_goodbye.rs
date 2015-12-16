extern crate effect_monad;

use effect_monad::EffectMonad;

fn main() {
    (|| {
        println!("Hello, world!");
    }).bind_ignore_contents(|| {
        println!("Goodbye, world!");
    })()
}
