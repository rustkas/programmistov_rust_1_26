#![feature(impl_trait_in_bindings)]

/*
cargo test --test
cargo test --test example2 -- --nocapture
*/

trait Trait: std::fmt::Debug {
    fn method(&self);
}

impl Trait for i32 {
    fn method(&self) {}
}

impl Trait for f32 {
    fn method(&self) {}
}

//    fn foo() -> ? {
//        5
//    }

#[test]
fn test1() {
    fn foo() -> Box<Trait> {
        Box::new(5) as Box<Trait>
    }

    let foo = foo();
    println!("{:?}", foo);
}
#[test]
fn test2() {
    fn foo() -> impl Trait {
        5
    }

    let foo: impl Trait = foo();
    println!("{:?}", foo);
}
