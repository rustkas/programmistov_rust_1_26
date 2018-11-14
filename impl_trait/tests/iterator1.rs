#![feature(impl_trait_in_bindings)]
#![feature(core_intrinsics)]
/*
cargo test --test
cargo test --test iterator1 test2 -- --nocapture
*/



fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

#[test]
fn test1() {
    fn foo() -> Vec<u8> {
        let value: Vec<u8> = vec![1, 2, 3]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0)
            .collect();
        value
    }
    println!("{:?}", foo());
}

#[test]
fn test2() {
    fn foo() -> Vec<u8> {
        let into_iter: std::vec::IntoIter<u8> = vec![1, 2, 3].into_iter();
        let map: std::iter::Map<std::vec::IntoIter<u8>, _> = into_iter.map(|x| x + 1);
        let filter: std::iter::Filter<std::iter::Map<std::vec::IntoIter<u8>, _>, _> =
            map.filter(|x| x % 2 == 0);
        let value: Vec<u8> = filter.collect();
        value
    }
    println!("{:?}", foo());
}

#[test]
fn test_imp1() {
    fn foo() -> impl Iterator<Item = u8> {
        vec![1, 2, 3]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0)
    }
    let filter: impl Iterator<Item = u8> = foo();
    let value: Vec<u8> = filter.collect();
    println!("{:?}", value);
}

#[test]
fn test() {
    //!    Здесь итератор фильтра может быть возвращен, а может и нет. Есть два разных типа,
    //!    которые могут быть возвращены, и поэтому мы должны использовать объект. - типаж
    fn foo(x: i32) -> Box<Iterator<Item = u8>> {

        let iter: std::iter::Map<std::vec::IntoIter<u8>, fn(u8) -> u8> =
            vec![1, 2, 3].into_iter().map(|x| x + 1);
        if x % 2 == 0 {
            Box::new(iter.filter(|x| x % 2 == 0))
        } else {
            Box::new(iter)
        }
    }
    let value1: Box<Iterator<Item = u8>> = foo(2);
    let value2: Box<Iterator<Item = u8>> = foo(3);
    print_type_of(&value1);
    print_type_of(&value2);
}
