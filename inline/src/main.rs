mod foo {
    // `pub` modifier is required to access `nested` outside of `foo`
    pub mod nested {
        // Brings `deeply_nested_fn` into the scope and re-exports it under `nested`
        pub use inner::deeply_nested_fn;

        mod inner {
            pub fn deeply_nested_fn() {
                println!("deeply_nested_fn");
            }
        }

        pub fn nested_fn() {
            println!("nested_fn");
        }
    }
}

mod bar {
    // `foo` is defined one level higher in the hierarchy.
    // `crate` points at the root module - in binary crates this is the `main.rs` file.
    // We could also use `super` to refer to the parent module which would be the same
    // in this case but using `crate` is more common.
    use crate::foo::nested::nested_fn;

    pub fn bar_fn() {
        println!("calling `nested_fn` from `bar_fn`");
        nested_fn()
    }
}

fn main() {
    foo::nested::nested_fn();
    // This would not compile because `inner` is private
    // module::nested::inner::deeply_nested_fn();
    foo::nested::deeply_nested_fn();
    bar::bar_fn();
}
