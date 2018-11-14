/*
cargo test -j 1 --test slice_patterns  -- --nocapture
*/

#[test]
fn test1() {
    println!("start test 1");
    let arr = [1, 2, 3];
    let arr2 = [10, 2, 3];
    match arr {
        [1, _, _] => println!("начинается с единицы"),
        [_a, _b, _c] => println!("начинается с чего-то другого"),
        //_ => println!(""),
    }

    match arr2 {
        [1, _, _] => println!("начинается с единицы"),
        [a, b, c] => println!("{} {} {}", a, b, c),
        //_ => println!(""),
    }
}

#[test]
fn test2() {
    fn foo(s: &[u8]) {
        match s {
            [a, b] => {
                println!("{} {}", a, b);
                ()
            }
            [a, b, c] => {
                println!("{} {} {}", a, b, c);
                ()
            }
            _ => (),
        }
    }
    foo(&[1, 2]);
    foo(&[1, 2, 3]);
}
