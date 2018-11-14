/*
cargo test -j 1 --test closed_rages  -- --nocapture
cargo test -j 1 --test closed_rages test4  -- --nocapture
*/

#[test]
fn test1() {
    println!("start test 1");
    for i in 1..3 {
        println!("i: {}", i);
    }
}

#[test]
fn test2() {
    println!("start test 2");
    for i in 1..=3 {
        println!("i: {}", i);
    }
}
#[test]
fn test3() {
    println!("start test 3");
    //    fn takes_u8(x: u8) {
    //        println!("{}", x);
    //    }
    //
    //    fn main() {
    //        for i in 0..256 {
    //            print!("i: {}; ", i);
    //            takes_u8(i);
    //        }
    //    }
    //
    //    main();
}
#[test]
fn test4() {
    println!("start test 4");
    fn takes_u8(x: u8) {
        println!("{}", x);
    }

    fn main() {
        for i in 0..=255 {
            if i <= 2 || i >= 254 {
                print!("i: {}; ", i);
                takes_u8(i);
            }
        }
    }

    main();
}
