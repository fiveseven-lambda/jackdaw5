mod ast;

fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        println!("{:#?}", ast::parse_expr(&s));
    }
}
