use crate::parse;

#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn main() {
        println!("TESTING NIGGA");
        parse::parse_template("\"String\"".to_string());
    }
}
