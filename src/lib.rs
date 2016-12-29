pub fn do_shit() -> String {
    "shit".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        assert_eq!("doShit", do_shit());
    }
}
