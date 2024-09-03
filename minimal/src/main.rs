mod foo {
    pub fn foo() {
        println!("foo");
    }
}

mod bar {
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
