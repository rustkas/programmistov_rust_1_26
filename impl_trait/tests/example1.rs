#![feature(impl_trait_in_bindings)]

/*
cargo test --test
cargo test --test example1
*/

// Using Traits study
#[test]
// using impl Trait
fn test1() {
    trait Trait {}
    impl Trait for () {}
    fn foo() -> impl Trait {
        // ...
    }
    let _fool: impl Trait = foo();
}

#[test]
// Tray to use trait-object
fn test2() {
    trait Trait {}

    // even do not compiled
    //    fn foo() -> Box<Trait> {
    //        // ...
    //    }
    //    let foo = foo();
}
