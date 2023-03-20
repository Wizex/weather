fn main() {
    if let Err(err) = weather::run() {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}
