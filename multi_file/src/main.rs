// `mod` declarations have to be placed at the root of the crate.
// In binary crates this is `main.rs`. In library crates it would be `lib.rs`.
mod foo;
mod bar;

fn main() {
    foo::nested::nested_fn();
    // This would not compile because `inner` is private.
    // foo::nested::inner::deeply_nested_fn();
    foo::nested::deeply_nested_fn();
    bar::bar_fn();
}
