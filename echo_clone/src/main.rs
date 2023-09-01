use std::env;
use echo_clone;

fn main() {
    let mut args: Vec<String> = env::args().collect();

//    println!("{:?}", args);

    echo_clone::remove_first(&mut args);

    let options: Vec<String> = echo_clone::check_for_option(args.clone());

    for option in options.clone() {
        if option.contains("e") {
            echo_clone::interpretate_echo(args.clone())
        }
    }

//    println!("{:?}", options);

//    echo_clone::default_echo(args);

}

