mod char;

fn main() {
    match char::read(&std::path::PathBuf::from("test")) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}
