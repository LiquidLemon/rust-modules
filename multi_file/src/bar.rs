use crate::foo::nested::nested_fn;

pub fn bar_fn() {
    println!("calling `nested_fn` from `bar_fn`");
    nested_fn()
}
