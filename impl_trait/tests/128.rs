/*
cargo test -j 1 --test 128  -- --nocapture
*/

#[test]
fn test1() {
    let x: i128 = 0;
    let y: u128 = 0;
    let sizeof_x = std::mem::size_of_val(&x);
    let sizeof_y = std::mem::size_of_val(&y);

    println!("{}", sizeof_x);
    println!("{}", sizeof_y);
}

#[test]
fn test2() {
    assert!(format!("{:02x?}", b"Foo\0") == "[46, 6f, 6f, 00]")
}
