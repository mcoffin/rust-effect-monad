(function() {var implementors = {};
implementors['effect_monad'] = ["impl&lt;T, Args&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html' title='core::ops::FnOnce'>FnOnce</a>&lt;Args&gt; for <a class='enum' href='effect_monad/enum.ResolveFn.html' title='effect_monad::ResolveFn'>ResolveFn</a>&lt;T&gt;","impl&lt;A, B, Ea, Eb, F&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html' title='core::ops::FnOnce'>FnOnce</a>&lt;()&gt; for <a class='struct' href='effect_monad/struct.BoundEffect.html' title='effect_monad::BoundEffect'>BoundEffect</a>&lt;Ea, F&gt; <span class='where'>where Ea: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html' title='core::ops::FnOnce'>FnOnce</a>() -&gt; A, Eb: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html' title='core::ops::FnOnce'>FnOnce</a>() -&gt; B, F: <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html' title='core::ops::FnOnce'>FnOnce</a>(A) -&gt; Eb</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
