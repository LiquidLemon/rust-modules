# Rust modules

While the Rust documentation is generally a great resource for learning about the language I find that it falls a little short when explaining how to split code into multiple files.
I had trouble understanding it when I was first learning the language and I know that it's a somewhat common pain point.
So here are a few practical examples of how to split your code into multiple files.

## Defining modules

Rust modules are similar to namespaces in other languages.
In all cases, a module must be explicitly defined using the `mod` keyword followed by the module name.
Usually, modules are placed in separate files but for now, I'm going to keep things simple and define two modules with all their contents in one place.

```rs
mod foo {
    // `pub` makes the function accessible outside of the module.
    pub fn foo() {
        println!("foo");
    }
}

mod bar {
    // `crate` references the root of our package (aka crate).
    use crate::foo::foo;

    pub fn bar() {
        print!("bar: ");
        foo();
    }
}

use bar::bar;

fn main() {
   bar();
}
```

Don't forget the `pub` modifier for anything that needs to be accessible outside of a module.
Otherwise, the compiler will complain, and `rust-analyzer` will not autocomplete names.

Notice the use of `crate`, which is a special name referencing the root of our package.
You'll use this pattern when you import things from other modules in the same package.
Alternatively, it's also possible to reference a parent module using `super` but it's less common.

As with standard library modules, `use` is optional.
Instead of permanently bringing names into the current scope you can specify the module path before the name.
In my example, we could remove the `use bar::bar` declaration and instead call the function as `bar::bar()` or even `crate::bar::bar`.

## Multiple files

Now let's see how this would look split into multiple files.

`src/main.rs`
```rs
mod foo;
mod bar;

use bar::bar;

fn main() {
   bar();
}
```

`src/foo.rs`
```rs
pub fn foo() {
    println!("foo");
}
```

`src/foo.rs`
```rs
use crate::foo::foo;

pub fn bar() {
    print!("bar: ");
    foo();
}
```

Instead of defining the contents of our modules in place in `main.rs` they're moved to separate files.
That's it, everything else is the same.

One somewhat surprising thing here might be that the `mod` declarations are still in `main.rs`.
`main.rs` is the root of our crate so this is where all top-level modules need to be specified.
In library crates, they would need to be placed in `lib.rs` instead.

## More complex example

Let's look at something more complex now.

`main.rs`
```rs
mod foo {
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
```

Here I introduced multiple levels to the module hierarchy.
There's almost no difference other than our module paths getting longer.
When using nested modules you also get granular control over the visibility of the modules themselves and their contents.

The only new thing here is the `pub use` declaration.
It works the same as before but also allows anything we import to be accessed by importing the module that contains the `use` declaration.

## Nested modules in multiple files

Now, how does all of this look when we split it into multiple files?

`main.rs`
```rs
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
```

`bar.rs`
```rs
use crate::foo::nested::nested_fn;

pub fn bar_fn() {
    println!("calling `nested_fn` from `bar_fn`");
    nested_fn()
}
```

`main.rs` stays mostly the same, but now it only contains two `mod` declarations for the top-level modules.
`bar` needed on extra treatment - I just moved its contents into a separate file, same as before.
I could do the same thing with `foo` but then the file would still contain a nested hierarchy of modules and that's not what I want.

There are two ways of handling nested modules and while in a real project, it would be best to be consistent, for the sake of example I'll show both approaches.

The `foo` module gets its own directory predictably called `foo`.
This directory contains a file called `foo/mod.rs` which acts as the root of that module.
This is important because while `foo` doesn't have any code of its own, it wraps another module which we have to declare.

`foo/mod.rs`
```rs
pub mod nested;
```

`nested` has yet another module nested inside of it but this time let's look at a different approach to nesting.
Next to `foo/mod.rs` we have both `foo/nested.rs` and `foo/nested/` (a directory).
The file will have all code which lives in `crate::foo::nested` while the directory is where we'll find anything that belongs to deeper nested modules.

`foo/nested.rs`
```rs
pub mod inner;

pub use inner::deeply_nested_fn;

pub fn nested_fn() {
    println!("nested_fn");
}
```

Now the only thing left is the `crate::foo::nested::inner` module which has no children so everything inside of it can be found in `foo/nested/inner.rs`.

`foo/nested/inner.rs`
```rs
pub fn deeply_nested_fn() {
    println!("deeply_nested_fn");
}
```

The second approach to nesting, which doesn't utilize `mod.rs` files, is what's recommended by the Rust team but it wasn't possible before Rust 1.30 so you might still run into many `mod.rs` files in the wild.

## Library crates

Libraries follow the same pattern of structuring code with the only difference being that the module tree starts at `lib.rs` instead of `main.rs`.
If you have both a `main.rs` and a `lib.rs` file (which you might, if your project is both a tool for end users and a library that exposes the same functionality to developers) things behave quite a bit differently.
When you declare modules in `lib.rs` they won't be available under `crate::` in `main.rs`.
You could work around this by importing your modules in both places but that's repetitive and error prone.
The preferred approach is to import your library using the name of the package (as defined in `Cargo.toml`).

`lib.rs`
```rs
pub fn foo() {
    println!("foo");
}
```

`main.rs`
```rs
use hybrid::foo;

fn main() {
    foo();
```

Keep in mind that this is a special case and accessing the root module by the package name is not possible in other scenarios (for example the name `hybrid` wouldn't be recognized in `lib.rs`).
