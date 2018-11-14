#![feature(core_intrinsics)]

/*
cargo test --test
cargo test --test iterator1 test2 -- --nocapture
*/



fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

#[test]
fn test1(){
     fn foo()->Vec<u8> {

         let value:Vec<u8> =    vec![1, 2, 3]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0).collect();
         value
    }
    println!("{:?}", foo());

}

#[test]
fn test2(){
    let into_iter:std::vec::IntoIter<i32> = vec![1, 2, 3]
        .into_iter();
    print_type_of(&into_iter);
//    let into_iter:IntoIter<i32> = vec![1, 2, 3]
//        .into_iter();
//
//    let map = into_iter.map(|x| x + 1);
//
//    let filter = vec![1, 2, 3]
//        .into_iter()
//        .map(|x| x + 1)
//        .filter(|x| x % 2 == 0);
}