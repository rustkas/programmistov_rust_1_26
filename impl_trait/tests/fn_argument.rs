#![feature(impl_trait_in_bindings)]

/*
cargo test --test
cargo test --test fn_argument  -- --nocapture
*/



#[test]
fn test1() {
    fn foo<T: std::fmt::Display>(_x: T) -> Vec<u8> {

        let value: Vec<u8> = vec![1, 2, 3]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0)
            .collect();
        value
    }
    println!("{:?}", foo(5));
}

#[test]
fn test2() {
        fn foo(_input: impl std::fmt::Display)  -> Vec<u8> {
            
        let into_iter: std::vec::IntoIter<u8> = vec![1, 2, 3].into_iter();
        let map: std::iter::Map<std::vec::IntoIter<u8>, _> = into_iter.map(|x| x + 1);
        let filter: std::iter::Filter<std::iter::Map<std::vec::IntoIter<u8>, _>, _> =
            map.filter(|x| x % 2 == 0);
        let value: Vec<u8> = filter.collect();
        value
    }
    println!("{:?}", foo(5));
}

