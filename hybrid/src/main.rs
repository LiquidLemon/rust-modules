// `lib.rs` creates a new module tree so we can't reference its root using `crate`.
// Instead the library is referenced by its name (defined by the package name in Cargo.toml)
use hybrid::foo;

// Technically it's also possible to import code from the library using it as a module.
// However, this approach doesn't seem to be commonly used.
// use lib::foo;

fn main() {
    foo();
}
