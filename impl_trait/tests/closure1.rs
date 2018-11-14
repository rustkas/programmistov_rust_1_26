#![feature(impl_trait_in_bindings)]
/*
cargo test --test
cargo test --test closure1 -- --nocapture
*/

#[test]
fn test1() {
    // было
    fn foo() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let fn_foo = foo();
    //    let fn_foo:Box<Fn<(i32), Output=i32>> = foo();
    println!("{}", fn_foo(12));
}

#[test]
fn test2() {
    // стало
    fn foo() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
    let fn_foo = foo();
    //let fn_foo:impl Fn<(i32), Output=i32> = foo();
    println!("{}", fn_foo(12));
}
