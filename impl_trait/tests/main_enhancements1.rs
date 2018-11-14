/*
cargo test --test
cargo test --test main_enhancements1  -- --nocapture
*/



use std::fs::File;


#[test]
fn test1(){
//    use std::fs::File;
//
//    fn main() {
//        let f = File::open("bar.txt")?;
//        f;
//    }
//
//    main();
}

#[test]
fn test2(){
//    use std::error::Error;
//    fn run(config: Config) -> Result<(), Box<Error>> {
//        // ...
//        Err(_)
//    }
//
//    fn main() {
//        // ...
//    let config = Config;
//        if let Err(e) = run(config) {
//            println!("Application error: {}", e);
//
//            use std::process;
//            process::exit(1);
//        }
//    }
}

#[test]
fn test3(){



    fn main() -> Result<(), std::io::Error> {
        let f = File::open("bar.txt")?;

        Ok(())
    }

}