/*
cargo test --test
cargo test --test match_enhancements1  -- --nocapture
*/

#[test]
fn test1() {
    fn hello(arg: &Option<String>) {
        match arg {
            Some(name) => println!("Hello {}!", name),
            None => println!("I don't know who you are."),
        }
    }

    hello(&Some("Rust".to_string()));
}

#[test]
fn test2() {
    //    fn hello(arg: &Option<String>) {
    //        match arg {
    //            &Some(name) => println!("Hello {}!", name),
    //            &None => println!("I don't know who you are."),
    //        }
    //    }
    //
    //    hello(&Some("Rust".to_string()));

}
#[test]
fn test3() {
    fn hello(arg: &Option<String>) {
        match arg {
            &Some(ref name) => println!("Hello {}!", name),
            &None => println!("I don't know who you are."),
        }
    }
    hello(&Some("Cargo".to_string()));
}

#[test]
fn test4() {
    fn hello(arg: &mut Option<String>) {
        match arg {
            Some(name) => name.push_str(", world"),
            None => (),
        }
    }
    hello(&mut Some("Cargo".to_string()));
}

#[test]
fn test5() {
    fn hello(arg: &mut Option<String>) {
        match arg {
            &mut Some(ref mut name) => name.push_str(", world"),
            &mut None => (),
        }
    }
    hello(&mut Some("Cargo".to_string()));
}
