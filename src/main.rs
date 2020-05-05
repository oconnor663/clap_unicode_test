use clap::{App, Arg};

fn main() {
    let matches = App::new("test")
        .arg(Arg::with_name("ARG").multiple(true))
        .get_matches();
    if let Some(args) = matches.values_of_os("ARG") {
        for arg in args {
            if arg.to_str().is_some() {
                println!("{:?} (valid Unicode)", arg);
            } else {
                println!("{:?} (INVALID Unicode!!!)", arg);
            }
        }
    }
}
